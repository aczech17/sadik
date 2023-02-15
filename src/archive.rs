use std::fs::File;
use std::io;
use std::io::{Read, Write};
use crate::file_info::{BUFSIZE, FileTreeInfo, METADATA_DELIMITER};

pub(crate) fn archive_files(file_tree_info: FileTreeInfo, archive_name: String) -> io::Result<()>
{
    let mut archive_file = File::create(archive_name)?;
    let delimiter: [u8; 1] = [METADATA_DELIMITER];

    // write metadata
    let meta_info = serde_json::to_string(&file_tree_info)?;
    archive_file.write_all((&meta_info).as_ref())?;
    archive_file.write(&delimiter)?;

    let file_info_vector = file_tree_info.get_file_infos();

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
