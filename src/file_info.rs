use std::fs::File;
use std::io;
use serde::{Serialize, Deserialize};

pub(crate) const BUFSIZE: usize = 1024;

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
pub(crate) struct FileInfoVector(pub(crate) Vec<FileInfo>);

impl FileInfoVector
{
    pub(crate) fn new(filenames: &[String]) -> Self
    {
        let mut file_info_vector: Vec<FileInfo> = Vec::new();
        for filename in filenames
        {
            let file_info = FileInfo::new(filename).unwrap();
            file_info_vector.push(file_info);
        }

        Self(file_info_vector)
    }
}