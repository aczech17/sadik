use std::fs;
use std::fs::{File, metadata};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::file_info::{FileInfo, FileTreeInfo};

mod file_info;
mod archive;
mod unarchive;
mod parse_args;

use crate::archive::archive_files;
use crate::unarchive::unarchive_files;
use crate::parse_args::{parse_options, ProgramOptions};
use walkdir::{DirEntry, WalkDir};


fn get_pathnames_and_filenames(args: Vec<String>) -> (Vec<String>, Vec<String>)
{
    let mut pathnames = Vec::new();
    let mut filenames = Vec::new();
    for arg in args
    {
        for dir in WalkDir::new(arg)
        {
            let dir = dir.unwrap();
            let path = dir.path().display().to_string();
            if metadata(path.clone()).unwrap().is_dir()
            {
                pathnames.push(path);
            }
            else
            {
                filenames.push(path);
            }
        }
    }

    (pathnames, filenames)
}

fn main()
{
    /*
    let args = vec!
    [
        "r".to_string(),
        "in1".to_string(),
        "in2".to_string(),
    ];

    let (paths, files) = get_pathnames_and_filenames(args);

    let info = FileTreeInfo::new(&paths[..], &files[..])
        .expect("Nie udało się zrobić tree info");

    archive_files(info, "out".to_string())
        .expect("Nie udało się spakować");
    */

    unarchive_files("out".to_string())
        .expect("Nie udało się rozpakować");
}
