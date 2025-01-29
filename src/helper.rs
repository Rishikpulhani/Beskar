use std::fs;
use std::fs::File;
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


pub fn run_remap_tool() {
    
    let remap_file_content_string =
        fs::read_to_string("./remappings.txt").expect("cannot read remappings.txt file");
    let remapped_dir_iter = remap_file_content_string.lines();
    for a_remapping in remapped_dir_iter {
       
        let dir_path_owned_template = String::from(a_remapping);
        let mut path_vector = dir_path_owned_template.split('=');
        path_vector.next();
        let dir_path_string: String = path_vector.next().unwrap().to_string();
        let test_dir_path_string: String = dir_path_string.replace("contracts", "test");
        
        let paths = fs::read_dir(&dir_path_string).unwrap();
      
        for path_ in paths {
            
            
            let path = path_.unwrap().path();
            let path_str = path.to_str().unwrap();
            //println!("{:?}",path);
            if path_str.ends_with(".sol") {
                
                let copy_file_name = path.file_name().unwrap().to_str().unwrap();
                let copy_file_path = format!("./src/{}", &copy_file_name);

                let file_name_without_extension =
                    copy_file_name.split(".").collect::<Vec<&str>>()[0];
                let test_copy_file_name = format!("{file_name_without_extension}.t.sol");
                let test_copy_file_path = format!("./test/{test_copy_file_name}");

                let test_file_path = format!("{test_dir_path_string}/{test_copy_file_name}");
                let file_copy = File::create(&copy_file_path).unwrap();
                fs::copy(&path, Path::new(&copy_file_path));
                let test_file_copy = File::create(&test_copy_file_path).unwrap();
                fs::copy(test_file_path, Path::new(&test_copy_file_path));
               
            } else {
                continue;
            }
        }
    }
}

pub fn run_tool(dir_path: &str, ctrlc_caller: Arc<AtomicBool>) {
    //println!("ran run_tool");
    let paths = fs::read_dir(dir_path).unwrap();
    let mut handles = Vec::new();
    let dir_path_owned: Arc<String> = Arc::new(String::from(dir_path));

    for path_ in paths {
        let r1 = ctrlc_caller.clone();
        let dir_path_th = dir_path_owned.clone();
        handles.push(thread::spawn(move || {
            let path = path_.unwrap().path();
            let new_file = PathBuf::from(path.clone());
            let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
            let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
            let file_path = format!("{dir_path_th}/{}", file_name);
            let tmp_file_name = format!("{dir_path_th}/{}tmp.sol", file_name);
            

            //mutant generation
            mutate(&path, &tmp_file_name, dir_path_th.clone());
            let mutants = fs::read_dir(format!(
                "./gambit_out_{}/mutants",
                file_name_without_extension
            ))
            .unwrap();
            for mutant in mutants {
                if r1.load(Ordering::SeqCst) {
                    let mutant_check = mutant.as_ref().unwrap().path();
                    let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
                    run_tests(
                        &mutant_dir,
                        &mutant_check,
                        &path,
                        dir_path_th.clone(),
                        false,
                    );
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
            generate_output(&mutant_dir, &path)
        }
    }
}

// change the beskar out folder division to fill folder wise datat from remappings
