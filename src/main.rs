mod crypto;
use crypto::{encrypt_file, decrypt_file};

fn main() {
    println!("Hello, world!");
    encrypt_file("testfiles/meow.txt", "testfiles/meow.enc").unwrap();
    decrypt_file("testfiles/meow.enc", "testfiles/meow.dec.text").unwrap();
}
