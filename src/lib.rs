use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use tar::Builder;
use xz2::write::XzEncoder;

// Credit: ChatGPT-4o
pub fn compress_directory_to_tar_xz(src_dir: String, dest_file: String) -> std::io::Result<()> {
    // Open the destination file
    let tar_xz_file = File::create(dest_file)?;

    // Wrap the file in a buffered writer for better performance
    let buf_writer = BufWriter::new(tar_xz_file);

    // Create an XZ encoder with the buffered writer
    let mut xz_encoder = XzEncoder::new(buf_writer, 9); // 9 is the compression level

    // Create a tar builder that writes to the XZ encoder
    {
        let mut tar_builder = Builder::new(&mut xz_encoder);

        // Append the directory to the tar archive
        tar_builder.append_dir_all(".", src_dir)?;

        // Finish the tar archive
        tar_builder.finish()?;
    } // tar_builder is dropped here, ending the borrow on xz_encoder

    // Finish the XZ encoding
    xz_encoder.finish()?;

    Ok(())
}



