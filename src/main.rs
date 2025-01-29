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
    let mut args: Args = std::env::args().into_iter(); // iterator
    args.next();
    
    // since this is an onwed value if we directly pass it to the lines method which takes a reference to this then once it has operated i upon the return value references this string but since it is an owned type and after the lines method there is no let on it i.e no variable owns this value so it will be dropped, so we use the let to own this string and later reference it
   
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

            // remapping run
            if (input_args.remap && fs::exists("./remappings.txt").unwrap()) {
                
                run_remap_tool();
                run_tool("./src", running.clone());

            } else {
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
