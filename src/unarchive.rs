use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read, Write};
use std::process::exit;
use crate::file_info::{FileInfo, BUFSIZE, FileTreeInfo, METADATA_DELIMITER};
use std::string::String;

fn save_file(file_info: &FileInfo, archive_file: &mut File) -> io::Result<()>
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

fn get_file_tree_info(archive_file: &mut File) -> io::Result<FileTreeInfo>
{
    let mut header_bytes: Vec<u8> = Vec::new();
    let mut byte: Vec<u8> = vec![0];

    while byte[0] != METADATA_DELIMITER
    {
        archive_file.read_exact(&mut byte)?;
        header_bytes.push(byte[0]);
    }

    header_bytes.pop(); // remove delimiter from metadata string

    let header = match String::from_utf8(header_bytes)
    {
        Ok(h) => h,
        Err(s) =>
            return Err(std::io::Error::new(ErrorKind::InvalidInput, s)),
        // ale koksztys
    };

    let file_tree_info: FileTreeInfo = match serde_json::from_str(&header)
    {
        Ok(f) => f,
        Err(s) =>
            {
                eprintln!("Nie udało się sparsować metadanych.");
                eprintln!("{}", header);
                exit(1);
                return Err(std::io::Error::new(ErrorKind::InvalidInput, s));
            }
        // ale koksztys
    };

    Ok(file_tree_info)
}

pub(crate) fn unarchive_files(archive_name: String) -> io::Result<()>
{
    let mut archive_file = File::open(archive_name)
        .expect("Could not open archive file");
    let file_tree_info = get_file_tree_info(&mut archive_file)
        .expect("Could not get file tree info");

    let dir_names = file_tree_info.get_dir_names();
    let file_info_vector = file_tree_info.get_file_infos();

    for dir_name in dir_names
    {
        fs::create_dir(dir_name)
            .expect(&*format!("Could not create directory {}", dir_name));
    }

    for file_info in file_info_vector
    {
        save_file(file_info, &mut archive_file)
            .expect(&*format!("Could not create file {}", file_info.get_name()));
    }

    Ok(())
}