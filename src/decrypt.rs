mod crypto;
use crypto::decrypt_directory;

fn main() {
    println!("Hello, world!");
    println!("Encrypting test directory");
    decrypt_directory("testdir").unwrap();
    
}
