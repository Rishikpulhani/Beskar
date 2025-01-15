use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;
use std::thread;

use ctrlc_async;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use Beskar::{generate_output, mutate, run_tests};

/// 1. terminal report overview
/*
----------------------------------------------
|                                            |
| mutant  number: **mutant number**          |
| running tests(processing)                  |
| testing completed                          |
| if passed: display [PASS] with green color |
| else: [FAIL] with reason in red color      |
| reason: passing tests info                 |
|                                            |
----------------------------------------------
 */

fn main() {
    /* assuming run from foundry project root */

    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "run" => {
            let paths = fs::read_dir("./src").unwrap();
            let mut handles = Vec::new();

            for path_ in paths {
                handles.push(thread::spawn(move || {
                    
                    let path = path_.unwrap().path();
                    let new_file = PathBuf::from(path.clone());
                    let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
                    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
                    let file_path = format!("./src/{}", file_name);
                    let tmp_file_name = format!("./src/{}tmp.sol",file_name);
                    // chnage required

                    mutate(&path, &tmp_file_name);
                    let mutants = fs::read_dir(format!("./gambit_out_{}/mutants",file_name_without_extension)).unwrap();
                    //fs::read_dir("./gambit_out/mutants").unwrap();
                    for mutant in mutants {
                        let mutant_check = mutant.as_ref().unwrap().path();
                        let mutant_dir =
                            mutant.as_ref().unwrap().file_name().into_string().unwrap();
                        run_tests(&mutant_dir, &mutant_check, &path);
                        //generate_output(&mutant_dir, &path)
                    }

                    let _ = fs::copy(Path::new(&tmp_file_name), Path::new(&file_path));
                    fs::remove_file(tmp_file_name);
                }));
                
                /*let tmp_file_name = format!("./src/{}", "tmp.sol");
                // chnage required
                let path = path_.unwrap().path();
                let new_file = PathBuf::from(path.clone());
                let file_name = new_file.file_name().unwrap().to_str().unwrap();//this gives the filename of the file currently being mutatated
                let file_path = format!("./src/{}", file_name);

                mutate(&path, &tmp_file_name);
                let mutants = fs::read_dir("./gambit_out/mutants").unwrap();
                for mutant in mutants {
                    let mutant_check = mutant.as_ref().unwrap().path();
                    let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
                    run_tests(&mutant_dir, &mutant_check, &path);
                    generate_output(&mutant_dir)
                }

                let _ = fs::copy(Path::new(&tmp_file_name),Path::new(&file_path));*/
            }
            for handle in handles {
                handle.join().unwrap();
            }
            let paths = fs::read_dir("./src").unwrap();
            for path_ in paths {
                let path = path_.unwrap().path();
                    let new_file = PathBuf::from(path.clone());
                    let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
                    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
                let mutants = fs::read_dir(format!("./gambit_out_{}/mutants",file_name_without_extension)).unwrap();
                for mutant in mutants {
                    let mutant_dir =
                            mutant.as_ref().unwrap().file_name().into_string().unwrap();
                    //println!("results for {}",file_name);
                    generate_output(&mutant_dir, &path)
                }
            }
        }
        _ => {
            println!("invalid command");
        }
    }
}
