mod crypto;
use crypto::{encrypt_file, decrypt_file};

fn main() {
    println!("Hello, world!");
    encrypt_file("meow.txt", "meow.enc").unwrap();
    decrypt_file("meow.enc", "meow.dec.text").unwrap();
}
