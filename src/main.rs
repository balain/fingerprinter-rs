/*
 * Process the provided folder and calculate SHA256 hashes for all files not in an exclude list
 *
 * Exclude list can match any of the directory hierarchy (but must fully match)
 *
 * Originally based on code from: https://stackoverflow.com/a/63543625
 */

use clap::Parser;
use rayon::prelude::*;

// Original Sha256 implementation
#[allow(unused_imports)]
use sha2::digest::Output;
#[allow(unused_imports)]
use sha2::{Digest, Sha256, Sha256VarCore};

// New xxhash implementation
#[allow(unused_imports)]
use xxhash_rust::xxh3::xxh3_64;

use std::fs::*;
use std::io;
use std::io::{Read, Write};
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, UNIX_EPOCH};
use std::{env, fs};
use chrono::{DateTime, Local};
use serde::Serialize;
#[allow(unused_imports)]
use serde_json::{json, Value};

/// Simple path processor
#[derive(Parser, Debug)]
#[command(version, about, name = ".")]
struct Args {
    /// Path to process
    #[arg(short, long)]
    path: String,
    #[arg(short, long, default_value = "./output.json")]
    output: String,
    #[arg(short, long, default_value = "false")]
    save_full_path: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
struct FileRecordSha256 {
    file: String,
    hash: Output<Sha256VarCore>,
    timestamp_mod: FileRecordsTimestamps,
    // timestamp_cre: FileRecordsTimestamps,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[allow(dead_code)]
struct FileRecordXxhash {
    file: String,
    hash: u64,
    timestamp_mod: FileRecordsTimestamps,
    // timestamp_cre: FileRecordsTimestamps,
}

#[derive(Serialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct FileRecordsTimestamps {
    epoch: u128,
    iso8601: String,
}

#[derive(Serialize)]
struct PathInfo {
    base: String,
    cmdline: String,
    run_from: String,
}

#[derive(Serialize)]
struct FileRecordsXxhash {
    updated_on: FileRecordsTimestamps,
    path: PathInfo,
    files: Vec<FileRecordXxhash>,
    count: usize,
}

// Originally based on https://stackoverflow.com/questions/63542762
// The Cookbook has some good suggestions: https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
// Tried jwalk but had issues with borrowing, so went back to a simpler implementation
fn visit(path: &Path, cb: &mut dyn FnMut(PathBuf), exclude: Vec<&str>) -> io::Result<()> {
    for e in read_dir(path)? {
        let e = e?;
        let path = e.path();

        // Hash processing based on https://github.com/RustCrypto/hashes example "Hashing Readable Objects"
        let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
        // Only include files that don't have a component in the exclude vector
        // logic from: https://stackoverflow.com/a/29504547
        let match_count = components
            .par_iter()
            .filter(|&x| exclude.contains(&x.to_str().unwrap()))
            .count();
        if match_count == 0 {
            if path.is_dir() {
                visit(&path, cb, exclude.clone())?;
            } else if path.is_file() {
                cb(path);
            }
        }
    }
    Ok(())
}

fn main() {
    let start: Instant = Instant::now(); // Start timer

    // Parse cli arguments
    let args = Args::parse();
    let pathname: String = args.path.clone();

    // To switch from Sha256 <-> Xxhash: 4 steps - #1: define filelist
    // Sha256
    // let filelist = Arc::new(Mutex::new(Vec::<FileRecordSha256>::new()));
    // Xxhash
    let filelist = Arc::new(Mutex::new(Vec::<FileRecordXxhash>::new()));

    // List of paths (full string) to exclude
    // If any one of these items is found in the full path, that entry will be ignored/excluded
    let exclude = vec![".idea", "target", ".git", "node_modules", "lib"];

    // let cwd = Path::new(pathname.as_str());
    // env::set_current_dir(cwd).unwrap();

    let path = Path::new(pathname.as_str());
    let mut files = Vec::new();
    visit(&path, &mut |e| files.push(e), exclude).unwrap();
    // Added Rayon to the iterator loop
    files.par_iter().for_each(|f| {
        // Multithreaded (uses Rayon)
        // Uncomment the next line if you don't want to use Rayon/multiple threads
        // files.iter().for_each(|f| {            // Single-threaded
        // https://github.com/RustCrypto/hashes/tree/master/sha2

        // TODO: Try to speed up calculating hashes of large files
        let f2 = File::open(f.clone());

        // To switch from Sha256 <-> Xxhash: 4 steps - #2: Set res to the right output
        // let res: Result<(&str, Output<Sha256VarCore>), bool> = match f2 { // Sha256
        let res: Result<(&str, u64), bool> = match f2 {
            // xxhash
            #[allow(unused_mut)]
            Ok(mut f2) => {
                // To switch from Sha256 <-> Xxhash: 4 steps - #3: Change the implementation block
                // Original Sha256 implementation
                // let mut hasher = Sha256::new();
                // let _n = io::copy(&mut &f2, &mut hasher);
                // let hash = hasher.finalize();
                // //println!("{:?}: {:x}", f, hash); // TODO: Change to JSON output
                // Result::Ok((
                //     <PathBuf as AsRef<Path>>::as_ref(f)
                //         .to_str()
                //         .unwrap_or_default(),
                //     hash,
                // ))

                // New xxhash implementation
                let mut buffer = Vec::new();
                buffer.clear();
                let _ = f2.read_to_end(&mut buffer);

                // Save full path?
                // Working relative path
                let filename = <PathBuf as AsRef<Path>>::as_ref(&f)
                    .to_str()
                    .unwrap_or_default();
                // Convert to absolute path
                // let filenameref = <PathBuf as AsRef<Path>>::as_ref(f);
                // let filename = filenameref.clone().canonicalize().unwrap().to_str().unwrap_or_default();
                // .clone().canonicalize().unwrap().to_str().unwrap_or_default();
                // let fullpath = Path::new(filename.to_owned().as_str()).canonicalize().unwrap().to_str().unwrap_or_default();
                // TODO: Fix check for save_full_path argument
                // if args.safe_full;path {
                // Result::Ok((Path::new(fullpath.clone()).clone().canonicalize().unwrap().to_str().unwrap(),
                //     xxhash_rust::xxh3::xxh3_64(&buffer)))
                // Result::Ok((filename, xxhash_rust::xxh3::xxh3_64(&buffer)))
                //
                // } else { // Only save relative path
                Result::Ok((filename, xxhash_rust::xxh3::xxh3_64(&buffer)))
                // }
            }
            Err(ref e) => {
                eprintln!("Error: (file: {:?}) {:?} ", f, e);
                Result::Err(false)
            }
        };
        if res.is_ok() {
            let metadata =
                fs::metadata(res.unwrap().0).expect("Unable to get metadata for file: @203");
            if let Ok(modtime) = metadata.modified() {
                let modtimemillis = modtime.duration_since(UNIX_EPOCH).unwrap().as_millis();
                // let cretimemillis = metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis();
                // Create a new struct instance and populate with filename and filehash
                // To switch from Sha256 <-> Xxhash: 4 steps - #4: Change the record hash impl
                // Sha256
                //     let frs = FileRecordSha256 {
                //         filename: res.unwrap().0.to_owned(),
                //         filehash: res.unwrap().1,
                //         timestamp_mod: FileRecordsTimestamps {
                //             epoch: time.duration_since(UNIX_EPOCH).unwrap().as_millis(),
                //             iso8601: DateTime::from_timestamp_millis(time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64).expect("Unable to parse timestamp").to_rfc3339(),
                //         },
                //     };
                //     filelist.lock().unwrap().push(frs);

                // Xxhash

                let frs = FileRecordXxhash {
                    file: res.unwrap().0.to_owned(),
                    hash: res.unwrap().1,
                    timestamp_mod: FileRecordsTimestamps {
                        epoch: modtimemillis,
                        iso8601: DateTime::from_timestamp_millis(modtimemillis as i64).expect("Unable to parse timestamp").with_timezone(&Local::now().timezone()).to_rfc3339(),
                    },
                    // timestamp_cre: FileRecordsTimestamps {
                    //     epoch: cretimemillis,
                    //     iso8601: DateTime::from_timestamp_millis(cretimemillis as i64).expect("Unable to parse timestamp").with_timezone(&Local::now().timezone()).to_rfc3339(),
                    // },
                };
                filelist.lock().unwrap().push(frs);
            }
        }
    });

    // Now sort and print vector (filelist)
    // TODO: Avoid creating a new variable to hold the list
    let mut fls2 = filelist.lock().unwrap().to_vec();
    fls2.sort(); // It just works!
    let path_info = PathInfo {
        base: path.canonicalize().unwrap().to_str().unwrap().to_owned(),
        cmdline: args.path.clone(),
        run_from: env::current_dir().unwrap().to_str().unwrap().to_owned(),
    };
    let file_record: FileRecordsXxhash = FileRecordsXxhash {
        path: path_info,
        updated_on: get_time().unwrap(),
        count: fls2.len(),
        files: fls2.clone(),
    };
    let file_record_json = serde_json::to_string(&file_record).unwrap();
    // for i2 in fls2.iter() {
    //     println!("{:?} == {:x}", i2.filename, i2.filehash);
    // }

    File::create(args.output)
        .unwrap()
        .write_all(file_record_json.as_bytes())
        .unwrap();

    let dur: Duration = start.elapsed(); // End timer
    eprintln!("Time elapsed: {:?}", dur); // Show elapsed time to STDERR
}

fn get_time() -> Result<FileRecordsTimestamps, std::io::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};

    /* std::time attempt - can't format rfc3339 :(
      so switched to chrono
    */
    // let start = SystemTime::now();
    // let since_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // Ok(FileRecordsTimestamps {
    //     epoch: since_epoch.as_millis(),
    //     iso8601: start.to_rfc3339(),
    //     // (SystemTime::duration_from_millis(since_epoch).to_rfc3339()).to_string(),
    // })

    use chrono::Local;

    Ok(FileRecordsTimestamps {
        epoch: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        iso8601: Local::now().to_rfc3339(),
    })
}
