use spinners::{Spinner, Spinners};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use std::fs::{self, create_dir, File};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn run_tests(
    mutant_dir: &String,
    mutant_check: &PathBuf,
    path: &PathBuf,
    dir_path_th: Arc<String>,
    remap: bool,
) {
    println!("running test functuion in test for {}",mutant_dir);
    ////path is contracts/governance/gov.sol
    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap(); //gov.sol
    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0]; //gov
    let test_file_name = format!("{}.t.sol", file_name_without_extension); //gov.t.sol

    let file_path = format!("{}/{}", dir_path_th, file_name); //./contracts/goveernance/gov.sol
                                                              //let file_path = format!("./src/{}", file_name);
                                                              //println!("{:?}", mutant_dir);
    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
    //println!("{:?}", mutant_vec);
    let mutant_num = mutant_vec[mutant_vec.len() - 1];
    let path_to_dir = path
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let path_to_parent = path.parent().unwrap().parent().unwrap().to_str().unwrap();

    let dir_name = String::from(path_to_dir);
    let mutant_file = format!("{}/{}/{}", mutant_check.display(), dir_name, file_name); //replace src

    let _ = fs::copy(Path::new(&mutant_file), Path::new(&file_path));
    //println!("Running Mutants of {}", file_name);;
    //println!("Mutant Number: {}", mutant_num);
    //let mut sp = Spinner::new(Spinners::Dots9, "running tests".into());
    //sleep(Duration::from_secs(3));

    let _ = create_dir(format!("./beskar_out_{}", file_name_without_extension));
    let out_file_path = format!(
        "./beskar_out_{}/outfile{}.txt",
        file_name_without_extension, mutant_num
    );
    let out_file = File::create(out_file_path.clone()).expect("failed to open output file.");
    //println!("//////////////{test_file_name}");
    let mut test_file_path = String::new();

    if remap {
        test_file_path = format!("{}/test/{}", path_to_parent, test_file_name);
        println!("{:?}", path);
        println!("{test_file_path}");
        println!("{:?}", path_to_parent);
        //let t_debug = PathBuf::from(&test_file_name);
        //println!("{:?}",t_debug);
        let t_debug_s = test_file_path.clone();
        let td = Path::new(&t_debug_s);
        println!("{:?}", td);
        println!("{t_debug_s}");
    } else {
        test_file_path = format!("test/{}", test_file_name);
    }
    let mut child = Command::new("forge")
        .args(["test", "--match-path", &test_file_path])
        .stdout(out_file)
        .spawn()
        .expect("failed to execute forge test");

    let _ = child.wait();
    //sp.stop();
    //println!();
}
