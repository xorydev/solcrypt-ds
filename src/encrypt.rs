mod crypto;
use crypto::encrypt_directory;

fn main() {
    println!("Hello, world!");
    println!("Encrypting test directory");
    encrypt_directory("testdir").unwrap();
    
}
