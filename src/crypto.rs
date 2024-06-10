extern crate aes;
extern crate block_modes;
extern crate hex;
extern crate rand;

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use std::str;


type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const KEY: &[u8] = b"keyhereshouldbereplacedbybuilder";
const IV: &[u8] = b"unique_initializ"; // IV should be 16 bytes
                                

pub fn encrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Read the input file
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Encrypt the data
    let cipher = Aes256Cbc::new_from_slices(KEY, IV)?;
    let ciphertext = cipher.encrypt_vec(&buffer);

    // Write the encrypted data to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&ciphertext)?;

    Ok(())
}

pub fn decrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Read the encrypted file
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Decrypt the data
    let cipher = Aes256Cbc::new_from_slices(KEY, IV)?;
    let decrypted_data = cipher.decrypt_vec(&buffer)?;

    // Write the decrypted data to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&decrypted_data)?;

    Ok(())
}
