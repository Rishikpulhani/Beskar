use ctrlc::set_handler;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use Beskar::{clear, generate_output, mutate, run_tests};

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
            // removal of earlier files due to previuos iteration if the tool
            clear();

            // mutant generation and testing of tests
            let paths = fs::read_dir("./src").unwrap();
            let mut handles = Vec::new();
            ///let mut ctrlc_handlers = Vec::new();
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl-C handler");

            for path_ in paths {
                let r1 = running.clone();
                handles.push(thread::spawn(move || {
                    let path = path_.unwrap().path();
                    let new_file = PathBuf::from(path.clone());
                    let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
                    let file_name_without_extension =
                        file_name.split(".").collect::<Vec<&str>>()[0];
                    let file_path = format!("./src/{}", file_name);
                    let tmp_file_name = format!("./src/{}tmp.sol", file_name);

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
                            let mutant_dir =
                                mutant.as_ref().unwrap().file_name().into_string().unwrap();
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

            //output generation
            if running.load(Ordering::SeqCst) {
                let paths = fs::read_dir("./src").unwrap();
                for path_ in paths {
                    let path = path_.unwrap().path();
                    let new_file = PathBuf::from(path.clone());
                    let file_name = new_file.file_name().unwrap().to_str().unwrap(); //this gives the filename of the file currently being mutatated
                    let file_name_without_extension =
                        file_name.split(".").collect::<Vec<&str>>()[0];
                    let mutants = fs::read_dir(format!(
                        "./gambit_out_{}/mutants",
                        file_name_without_extension
                    ))
                    .unwrap();
                    for mutant in mutants {
                        let mutant_dir =
                            mutant.as_ref().unwrap().file_name().into_string().unwrap();
                        //println!("results for {}",file_name);
                        generate_output(&mutant_dir, &path)
                    }
                }
            } else {
                clear();
            }
        }

        _ => {
            println!("invalid command");
        }
    }
}
