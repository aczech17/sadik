use crate::file_info::FileInfoVector;

mod file_info;
mod archive;
mod unarchive;
mod parse_args;

use crate::archive::archive_files;
use crate::unarchive::unarchive_files;
use crate::parse_args::{parse_options, ProgramOptions};

fn main()
{
    let usage = format!("Super archiwizator i dearchiwizator kurwo\n\
    [-a [file names] [archive name]] | [-d [archive name]]");

    let args: Vec<String> = std::env::args().collect();

    match parse_options(args)
    {
        ProgramOptions::Archive { file_names, archive_filename } =>
        {
            let file_info_vector = match FileInfoVector::new(&file_names)
            {
                Ok(f) => f,
                Err(e) =>
                {
                    eprintln!("Could not find files.");
                    eprintln!("{}", e.to_string());
                    return;
                }
            };
            match archive_files(file_info_vector, archive_filename)
            {
                Ok(_) => {},
                Err(_) =>
                {
                    eprintln!("Could not archive files.");
                    return;
                }
            }
        }
        ProgramOptions::Unarchive { archive_filename } =>
        {
            match unarchive_files(archive_filename.clone())
            {
                Ok(_) => {},
                Err(e) =>
                {
                    eprintln!("Could not unpack archive named: {}", archive_filename);
                    eprintln!("{}", e.to_string());
                    return;
                }
            }
        }
        ProgramOptions::Invalid =>
        {
            eprintln!("{}", usage);
        }
    }
}
