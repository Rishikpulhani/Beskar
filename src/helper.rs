use std::fs;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crate::cli::*;
use crate::mutations::*;
use crate::tests::*;

pub fn clear() {
    let rm_dir_paths = fs::read_dir("./").unwrap();
    for p in rm_dir_paths.filter(|p| {
        let new_name = p.as_ref().unwrap().path();
        let new_file_name = new_name.file_name().unwrap().to_str().unwrap();

        //does the as_str did not copy the value?

        //No, this is exactly the difference between String and &str. String owns the value, &str is only a reference. &str is incapable of owning a value, it only references the String. And because you didn't store the String in a variable, it gets dropped while the &str still exists, which leads to the error.
        String::from(new_file_name).starts_with("gambit_out")
            || new_file_name.starts_with("beskar_out")
    }) {
        fs::remove_dir_all(p.unwrap().path()).unwrap();
    }
}

pub fn run_tool(dir_path: &'static str, ctrlc_caller: Arc<AtomicBool>) {
    let paths = fs::read_dir(dir_path).unwrap();
    let mut handles = Vec::new();

    for path_ in paths {
        let r1 = ctrlc_caller.clone();
        handles.push(thread::spawn(move || {
            let path = path_.unwrap().path();
            let new_file = PathBuf::from(path.clone());
            let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
            let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
            let file_path = format!("{dir_path}/{}", file_name);
            let tmp_file_name = format!("{dir_path}/{}tmp.sol", file_name);

            //mutant generation
            mutate(&path, &tmp_file_name);
            let mutants = fs::read_dir(format!(
                "./gambit_out_{}/mutants",
                file_name_without_extension
            ))
            .unwrap();
            //fs::read_dir("./gambit_out/mutants").unwrap();

            //tests running
            //looping
            for mutant in mutants {
                if r1.load(Ordering::SeqCst) {
                    let mutant_check = mutant.as_ref().unwrap().path();
                    let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
                    run_tests(&mutant_dir, &mutant_check, &path);
                //generate_output(&mutant_dir, &path)
                } else {
                    break;
                }
            }

            let _ = fs::copy(Path::new(&tmp_file_name), Path::new(&file_path));
            fs::remove_file(tmp_file_name).unwrap();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn print_result(dir_path: &'static str) {
    let paths = fs::read_dir(dir_path).unwrap();
    for path_ in paths {
        let path = path_.unwrap().path();
        let new_file = PathBuf::from(path.clone());
        let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
        let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
        let mutants = fs::read_dir(format!(
            "./gambit_out_{}/mutants",
            file_name_without_extension
        ))
        .unwrap();
        for mutant in mutants {
            let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
            //println!("results for {}",file_name);
            generate_output(&mutant_dir, &path)
        }
    }
}



// change the beskar out folder division to fill folder wise datat from remappings
