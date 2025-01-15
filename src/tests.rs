use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

use std::fs::{self, create_dir, File};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn run_tests(mutant_dir: &String, mutant_check: &PathBuf, path: &PathBuf) {
    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0];
    let test_file_name =  format!("{}.t.sol",file_name_without_extension);
    let file_path = format!("./src/{}", file_name);
    //println!("{:?}", mutant_dir);
    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
    //println!("{:?}", mutant_vec);
    let mutant_num = mutant_vec[mutant_vec.len() - 1];
    let mutant_file = format!("{}/src/{}", mutant_check.display(), file_name);

    let _ = fs::copy(Path::new(&mutant_file), Path::new(&file_path));
    //println!("Running Mutants of {}", file_name);;
    //println!("Mutant Number: {}", mutant_num);
    //let mut sp = Spinner::new(Spinners::Dots9, "running tests".into());
    //sleep(Duration::from_secs(3));

    let _ = create_dir(format!("./beskar_out_{}",file_name_without_extension));
    let out_file_path = format!("./beskar_out_{}/outfile{}.txt", file_name_without_extension,mutant_num);
    let out_file = File::create(out_file_path.clone()).expect("failed to open output file.");
    //println!("//////////////{test_file_name}");
    let test_file_path = format!("test/{}",test_file_name);
    let mut child = Command::new("forge")
        .args(["test","--match-path",&test_file_path])
        .stdout(out_file)
        .spawn()
        .expect("failed to execute forge test");

    let _ = child.wait();
    //sp.stop();
    //println!();
}
