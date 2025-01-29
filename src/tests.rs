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
    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap(); 
    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0]; 
    let test_file_name = format!("{}.t.sol", file_name_without_extension); 

    let file_path = format!("{}/{}", dir_path_th, file_name); 
    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
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
        let t_debug_s = test_file_path.clone();
        let td = Path::new(&t_debug_s);
        
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
