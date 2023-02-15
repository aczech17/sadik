use std::fs::File;
use std::io;
use serde::{Serialize, Deserialize};

pub(crate) const BUFSIZE: usize = 1024;
pub(crate) const METADATA_DELIMITER: u8 = 2; // ascii - STX (start of text)

#[derive(Serialize, Deserialize)]
pub(crate) struct FileInfo
{
    name: String,
    size: usize,
}

impl FileInfo
{
    pub(crate) fn new(filename: &str) -> io::Result<Self>
    {
        let file = File::open(filename)?;
        let size = file.metadata()?.len() as usize;

        Ok(Self
        {
            name: filename.to_string(),
            size,
        })
    }

    pub(crate) fn get_name(&self) -> &String
    {
        &self.name
    }
    pub(crate) fn get_size(&self) -> &usize
    {
        &self.size
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct FileTreeInfo
{
    dir_names: Vec<String>,
    file_infos: Vec<FileInfo>,
}

impl FileTreeInfo
{
    pub(crate) fn new(dir_names: &[String], filenames: &[String]) -> io::Result<Self>
    {
        let mut file_info_vector: Vec<FileInfo> = Vec::new();
        for filename in filenames
        {
            let file_info = FileInfo::new(filename)?;
            file_info_vector.push(file_info);
        }

        Ok(Self
        {
            dir_names: Vec::from(dir_names),
            file_infos: file_info_vector,
        })
    }

    pub(crate) fn get_file_infos(&self) -> &Vec<FileInfo>
    {
        &self.file_infos
    }
    pub(crate) fn get_dir_names(&self) -> &Vec<String>
    {
        &self.dir_names
    }
}
