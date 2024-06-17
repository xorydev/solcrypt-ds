mod crypto;
use crypto::decrypt_directory;
use std::env;

fn main() {
    let home_dir = env::var("USERPROFILE").unwrap_or("C:\\Users\\Xory".to_string());
    decrypt_directory(&home_dir).unwrap();
    
}
