use spinners::{Spinner, Spinners};
use std::fmt::format;
use std::thread::sleep;
use std::time::Duration;

use std::fs::{self, File, create_dir};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn run_tests(mutant_dir:&String, mutant_check: &PathBuf, path: &PathBuf, new_name_gambit : &str,index :usize){
    println!("running tests");
    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    let file_path = format!("./src/{}", file_name);
    
    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
    let mutant_num = mutant_vec[mutant_vec.len() - 1];
    let mutant_file = format!("{}/src/{}", mutant_check.display(), file_name);

    let _ = fs::copy(Path::new(&mutant_file), Path::new(&file_path));

    println!("Mutant Number: {}", mutant_num);
    println!("file number {}",&new_name_gambit);
    let mut sp = Spinner::new(Spinners::Dots9, "running tests".into());
    sleep(Duration::from_secs(3));
    let beskarout_name = format!("./beskar_out{}",index); 
    let _ = create_dir(beskarout_name);
    
    let out_file_path = format!("./beskar_out{}/outfile{}.txt", index,mutant_num);
    let out_file = File::create(out_file_path.clone()).expect("failed to open output file.");

    let test_srcfile_name = file_name.split(".").collect::<Vec<&str>>();
    let path_to_test_file = format!("./test/{}.t.sol", test_srcfile_name[0]);


    let mut child = Command::new("forge")
        .args(&["test", "--match-path", path_to_test_file.as_str()])
        .stdout(out_file)
        .spawn()
        .expect("failed to execute forge test");

    let _ = child.wait();
    sp.stop();
    println!();
}