use std::fs::{self, create_dir, File};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

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

    /*read solidity source files  */
    let paths = fs::read_dir("./src").unwrap();

    for path in paths {
        let new_file = PathBuf::from(path.unwrap().path());
        let file_name = new_file.file_name().unwrap().to_str().unwrap();
        let file_path = format!("./src/{}", file_name);
        if file_name.ends_with(".sol") {
            let output = Command::new("gambit")
                .args(["mutate", "--filename", file_path.as_str()])
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stdout));

            let tmp_file_name = format!("./src/{}", "tmp.sol");
            let _ = File::create(&tmp_file_name).unwrap();
            let _ = fs::copy(Path::new(&file_path), Path::new(&tmp_file_name));
            let mutants = fs::read_dir("./gambit_out/mutants").unwrap();

            for mutant in mutants {
                let mutant_check = mutant.as_ref().unwrap();
                let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
                let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
                let mutant_num = mutant_vec[mutant_vec.len() - 1];
                let mutant_file = format!("{}/src/{}", mutant_check.path().display(), file_name);

                let _ = fs::copy(Path::new(&mutant_file), Path::new(&file_path));
                let _ = create_dir("./beskar_out");
                let out_file_path = format!("./beskar_out/outfile{}.txt", mutant_num);
                let out_file =
                    File::create(out_file_path.clone()).expect("failed to open output file.");

                let mut child = Command::new("forge")
                    .args(["test"])
                    .stdout(out_file)
                    .spawn()
                    .expect("failed to execute forge test");

                let _ = child.wait();

                let output3 = Command::new("grep")
                    .args(["PASS", out_file_path.as_str()])
                    .output()
                    .expect("failed to grep");

                let final_op = String::from_utf8_lossy(&output3.stdout);
                if final_op == "" {
                    println!("mutant number : {} PASSED", mutant_num);
                } else {
                    println!("{}", final_op);
                }
            }
        }
    }
}
