mod crypto;
use crypto::decrypt_directory;
use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    let home: String = env::var("USERPROFILE").unwrap_or("C:\\Users\\Xory".to_string()); // no way this could fail!
    #[cfg(target_os = "linux")]
    let home: String = env::var("HOME").unwrap();
    decrypt_directory(&home_dir).unwrap();
    
}
