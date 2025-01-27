use std::env::Args;
use std::fs::{self};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use Beskar::{clear, print_result, run_remap_tool, run_tool, Cli_args};

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
    // the dot in the cli command foor running means any arguments that are there further are ar okay
    let mut args: Args = std::env::args().into_iter(); // iterator
    args.next();
    //let command = &args.next().unwrap();
    //let remap;
    //let remap_file_iter_res;
    //let mut dir_path_remap_vec = Vec::new();
    // since this is an onwed value if we directly pass it to the lines method which takes a reference to this then once it has operated i upon the return value references this string but since it is an owned type and after the lines method there is no let on it i.e no variable owns this value so it will be dropped, so we use the let to own this string and later reference it
    /*let remap_file_path = match args.next(){
        Some(val)=> if val.ends_with(".txt") {
            remap = true;
            remap_file_iter_res = fs::read_to_string(&val).unwrap();
            dir_path_remap_vec = remap_file_iter_res.lines().collect();
            val
        }
        else {
            remap = false;
            "none".to_string()
        },
        None => {
            remap=false;
            "none".to_string()
        },
    };
    println!("{remap}");*/
    //let input_args = Cli_args::new(command,remap,&remap_file_path);
    let input_args = Cli_args {
        command: &args.next().unwrap(),
        remap: match args.next() {
            Some(val) => {
                if val == "remappings" {
                    true
                } else {
                    false
                }
            }
            None => false,
        },
    };
    println!("{}",input_args.remap);
    match input_args.command {
        "run" => {
            // removal of earlier files due to previuos iteration if the tool
            clear();

            //ctrlc handler
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl-C handler");

            // mutant generation and testing of tests
            

            // remapping run
            if input_args.remap {
                println!("run");
                //let remap_file_path = String::from(input_args.remap_file_path);
                /*let dir_paths_res = fs::read_to_string("./remappings.txt").unwrap();
                let mut other_dir_paths = Vec::new();
                for path in dir_paths_res.lines(){
                    other_dir_paths.push(String::from(path));
                }
                run_remap_tool(other_dir_paths, running.clone());*/
                run_tool("./src", running.clone());
                run_remap_tool(running.clone());
            }
            else {
                run_tool("./src", running.clone());
            }

            //output generation
            if running.load(Ordering::SeqCst) {
                print_result("./src");
            } else {
                clear();
            }
        }

        _ => {
            println!("invalid command");
        }
    }
}
