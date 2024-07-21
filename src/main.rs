use std::time::{Duration, Instant};
use std::path::PathBuf;
use jwalk::Parallelism;
use jwalk::rayon::iter::ParallelIterator;
use jwalk::rayon::iter::ParallelBridge;

fn include_file<'a>(p: &PathBuf, exclude: &Vec<&str>, md5st: Vec<&'a str>) -> (bool, Vec<&'a str>) {
    let components: Vec<_> = p.components().map(|comp| comp.as_os_str()).collect();
    // Option A: Process if the first path element is NOT in exclude[]
    // if !exclude.contains(&components[1].to_str().unwrap()) {
    //     //println!("{:?}", p.as_os_str());
    // }

    // Option B: Process if NO exclude[] element is anywhere in the path
    // N.B. count > 0 means there's at least one match between them
    // https://stackoverflow.com/a/29504547

    let match_count= components.iter().filter(|&x| exclude.contains(&x.to_str().unwrap())).count();
    if match_count == 0 {
        // No matches, so process
        // println!("Processing {:?}", p.as_os_str());
        (true, md5st)
    } else {
        // println!("Skipping {:?}", p.as_os_str());
        (false, md5st)
    }
}

fn go<'a>(exclude:Vec<&'a str>,  md5st: &mut Vec<&'a str>) -> (Vec<&'a str>, usize) {
    let ctr:usize;
    ctr = jwalk::WalkDir::new(".")
        .parallelism(Parallelism::RayonNewPool(0))
        .into_iter()
        .par_bridge()
        .filter_map(|dir_entry_result| {
            let dir_entry = dir_entry_result.ok()?;
            if dir_entry.file_type().is_file() {
                let path: PathBuf = dir_entry.path();
                let infile: bool;
                (infile, *md5st) =  include_file(&path, &exclude, md5st.clone()); // Error here
                if infile {
                    println!("Calculating md5 for {:?}", path.as_os_str());
                    let fname: &str = &path.to_str().unwrap_or("undef_result_a"); // Error here
                    md5st.push(fname.clone()); // Error here
                    return Some(true)
                } else {
                    // println!("Skipping file: {:?}", path.as_os_str());
                    return Some(false)
                }
            }
            None
        }).count();
    (md5st.clone(), ctr)
}

fn main() {
    let mut md5st:Vec<&str> = Vec::new();
    // MD5ST.clone().push(  String::as_mut_str( &mut String::from("asdf")));
    let start: Instant = Instant::now();

    let exclude_list:Vec<&str> = vec!["src", "raw", "lib", "target", "node_modules"];
    // let first_entry = String::from("asdf");
    // md5st.push(&*first_entry);

    let result_count:usize;
    (md5st, result_count) = go(exclude_list, &mut md5st.to_owned());
    println!("Count: {:?}", result_count);
    println!("md5 vec len = {:?}",  md5st.len());

    let dur: Duration = start.elapsed();
    println!("Time elapsed: {:?}", dur);
}