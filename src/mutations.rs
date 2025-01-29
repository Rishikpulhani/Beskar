use std::fmt::format;
use std::fs::{self, File};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

pub fn mutate(path: &PathBuf, tmp_file_name: &String, dir_path_th: Arc<String>) {
    
    let new_file = PathBuf::from(path); 
    let file_name = new_file.file_name().unwrap().to_str().unwrap(); 
    let file_name_without_extension = file_name.split(".").collect::<Vec<&str>>()[0]; 
    let file_path = format!("{}/{}", dir_path_th, file_name); 

    if file_name.ends_with(".sol") {
        
        let output = Command::new("gambit")
            .args(["mutate", "--filename", file_path.as_str()])
            .output()
            .expect("failed to execute process");
        println!(
            "{} for {}",
            String::from_utf8_lossy(&output.stdout),
            file_name
        );
        fs::rename(
            "gambit_out",
            format!("gambit_out_{}", file_name_without_extension),
        )
        .unwrap();

        let _ = File::create(&tmp_file_name).unwrap();
        let _ = fs::copy(Path::new(&file_path), Path::new(&tmp_file_name));
    }
}
