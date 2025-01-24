use ctrlc::set_handler;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use Beskar::{clear, generate_output, print_result, run_tool};

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

            //ctrlc handler
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl-C handler");

            // mutant generation and testing of tests
            run_tool("./src", running.clone());

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
