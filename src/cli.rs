use colored::*;
use std::path::PathBuf;
use std::process::Command;

pub fn generate_output(mutant_dir: &String, path: &PathBuf) {
    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
    let mutant_num = mutant_vec[mutant_vec.len() - 1];
    let out_file_path = format!(
        "./beskar_out_{}/outfile{}.txt",
        file_name_without_extension, mutant_num
    );

    let output = Command::new("grep")
        .args(["PASS", out_file_path.as_str()])
        .output()
        .expect("failed to grep");

    let final_op = String::from_utf8_lossy(&output.stdout);
    let final_op_vec = final_op.split("[PASS]").collect::<Vec<&str>>();
    if final_op == "" {
        println!("ran tests for {}", file_name);
        println!("{} {}", "[PASS] mutant number".green(), mutant_num.green());
    } else {
        println!("ran tests for {}", file_name);
        println!("{} {}", "[FAIL] mutant number".red(), mutant_num.red());
        println!("{}", "Passing tests:".red());
        for i in 0..final_op_vec.len() {
            let passed_test = final_op_vec[i];
            println!("{}", passed_test.red());
        }
    }
}

pub struct Cli_args<'a> {
    pub command: &'a str,
    pub remap: bool,
}
