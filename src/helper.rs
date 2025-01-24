use std::fs;

pub fn clear() {
    let rm_dir_paths = fs::read_dir("./").unwrap();
    for p in rm_dir_paths.filter(|p| {
        let new_name = p.as_ref().unwrap().path();
        let new_file_name = new_name.file_name().unwrap().to_str().unwrap();

        //does the as_str did not copy the value?

        //No, this is exactly the difference between String and &str. String owns the value, &str is only a reference. &str is incapable of owning a value, it only references the String. And because you didn't store the String in a variable, it gets dropped while the &str still exists, which leads to the error.
        String::from(new_file_name).starts_with("gambit_out")
            || new_file_name.starts_with("beskar_out")
    }) {
        fs::remove_dir_all(p.unwrap().path()).unwrap();
    }
}
