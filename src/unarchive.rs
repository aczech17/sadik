use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read, Write};
use crate::file_info::{FileInfo, BUFSIZE, FileInfoVector};
use std::string::String;

fn save_file(file_info: FileInfo, archive_file: &mut File) -> io::Result<()>
{
    let size = file_info.get_size();
    let mut saved_file = File::create(file_info.get_name())?;
    let mut bytes_left = *size;

    while bytes_left > 0
    {
        let bytes_to_save = if bytes_left > BUFSIZE
        {
            BUFSIZE
        }
        else { bytes_left };

        let mut buf: Vec<u8> = vec![0; bytes_to_save];

        archive_file.read_exact(&mut buf)?;

        saved_file.write(&mut buf)?;

        bytes_left -= bytes_to_save;
    }

    Ok(())
}

fn get_file_info_vector(archive_file: &mut File) -> io::Result<Vec<FileInfo>>
{
    let mut header_bytes: Vec<u8> = Vec::new();
    let mut byte: Vec<u8> = vec![0];

    while byte[0] != b']'
    {
        archive_file.read_exact(&mut byte)?;
        header_bytes.push(byte[0]);
    }

    let header = match String::from_utf8(header_bytes)
    {
        Ok(h) => h,
        Err(s) =>
            return Err(std::io::Error::new(ErrorKind::InvalidInput, s)),
        // ale koksztys
    };

    let file_info_vector: FileInfoVector = match serde_json::from_str(&header)
    {
        Ok(f) => f,
        Err(s) =>
            return Err(std::io::Error::new(ErrorKind::InvalidInput, s)),
        // ale koksztys
    };

    Ok(file_info_vector.0)
}

pub(crate) fn unarchive_files(archive_name: String) -> io::Result<()>
{
    let mut archive_file = File::open(archive_name)?;
    let file_info_vector = get_file_info_vector(&mut archive_file)?;

    for file_info in file_info_vector
    {
        save_file(file_info, &mut archive_file)?;
    }

    Ok(())
}