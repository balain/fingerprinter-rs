/*
 * Process the provided folder and calculate SHA256 hashes for all files not in an exclude list
 *
 * Exclude list can match any of the directory hierarchy (but must fully match)
 *
 * Originally based on code from: https://stackoverflow.com/a/63543625
 */

use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
use std::fs::*;
use std::io;

use sha2::{Sha256, Digest};
use clap::Parser;
use rayon::prelude::*; // for parallel iter

/// Simple path processor
#[derive(Parser, Debug)]
#[command(version, about, name = ".")]
struct Args {
    /// Path to process
    #[arg(short, long)]
    pathname: String,
}

// Originally based on https://stackoverflow.com/questions/63542762
// The Cookbook has some good suggestions: https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
// Tried jwalk but had issues with borrowing, so went back to a simpler implementation
fn visit(path: &Path, cb: &mut dyn FnMut(PathBuf), exclude:Vec<&str>) -> io::Result<()> {
    for e in read_dir(path)? {
        let e = e?;
        let path = e.path();

        // Hash processing based on https://github.com/RustCrypto/hashes example "Hashing Readable Objects"
        let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
        // Only include files that don't have a component in the exclude vector
        // logic from: https://stackoverflow.com/a/29504547
        let match_count= components.iter().filter(|&x| exclude.contains(&x.to_str().unwrap())).count();
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
    let pathname:String = args.pathname;

    // List of paths (full string) to exclude
    // If any one of these items is found in the full path, that entry will be ignored/excluded
    let exclude = vec![".idea", "target", ".git", "node_modules", "lib"];

    let path = Path::new(pathname.as_str());
    let mut files = Vec::new();
    visit(path, &mut |e| files.push(e), exclude).unwrap();
    // Added Rayon to the iterator loop
    files.par_iter().for_each(|f| { // Multi-threaded (uses Rayon)
        // Uncomment the next line if you don't want to use Rayon/multiple threads
        // files.iter().for_each(|f| {            // Single-threaded
        // https://github.com/RustCrypto/hashes/tree/master/sha2

        // TODO: Try to speed up calculating hashes of large files
        let f2 = File::open(f.clone());
        let _res:Result<bool,bool> = match f2 {
            Ok(f2) => {
                let mut hasher = Sha256::new();
                let _n = io::copy(&mut &f2, &mut hasher);
                let hash = hasher.finalize();
                println!("{:?}: {:x}", f, hash); // TODO: Change to JSON output
                Result::Ok(true)
            },
            Err( ref e) => {
                eprintln!("Error: (file: {:?}) {:?}", f, e);
                Result::Err(false)
            }
        };
    });

    let dur: Duration = start.elapsed();  // End timer
    eprintln!("Time elapsed: {:?}", dur); // Show elapsed time to STDERR
}
