use std::fs::File;
use std::io;
use std::io::{Read, Write};
use crate::file_info::{BUFSIZE, FileInfoVector};

pub(crate) fn archive_files(file_info_vector: FileInfoVector, archive_name: &str) -> io::Result<()>
{
    let mut archive_file = File::create(archive_name)?;

    // write metada
    let meta_info = serde_json::to_string(&file_info_vector).unwrap();
    archive_file.write_all((&meta_info).as_ref()).unwrap();

    let file_info_vector = file_info_vector.0; // unpack the vector

    let mut buff: [u8; BUFSIZE] = [0; BUFSIZE];
    for file_info in file_info_vector
    {
        let filename = file_info.get_name();
        let mut file = File::open(filename)?;

        'file: loop
        {
            let n = file.read(&mut buff)?;
            if n == 0
            {
                break 'file;
            }

            archive_file.write_all(&buff[0..n])?;
        }
    }

    Ok(())
}
