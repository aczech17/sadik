mod file_info;
mod archive;
mod unarchive;
mod parse_args;

use crate::file_info::FileTreeInfo;
use crate::archive::archive_files;
use crate::unarchive::unarchive_files;
use crate::parse_args::{parse_options, ProgramOptions};

fn main()
{
    let args = std::env::args().collect();
    match parse_options(args)
    {
        ProgramOptions::Archive{pathnames, filenames, archive_filename}
        =>
        {
            let file_tree_info =
                match FileTreeInfo::new(&pathnames[..], &filenames[..])
                {
                    Ok(ft) => ft,
                    Err(_) =>
                    {
                        eprintln!("Could not make file tree info.");
                        return;
                    }
                };
            match archive_files(file_tree_info, archive_filename.clone())
            {
                Ok(_) => {}
                Err(e) =>
                {
                    eprintln!("Could not create the archive file {}: {}",
                              archive_filename, e.to_string());
                    return;
                }
            }
        }

        ProgramOptions::Unarchive { archive_filename } =>
        {
            match unarchive_files(archive_filename.clone())
            {
                Ok(_) => {}
                Err(e) =>
                {
                    eprintln!("Could not extract the archive file {}: {}",
                        archive_filename, e.to_string());
                    return;
                }
            }
        }
        ProgramOptions::Invalid =>
        {
            eprintln!("Super archiwizator i dearchiwizator kurwo");
            eprintln!("sadik [-a [file names] [archive name]] | [-d [archive name]]");
            return;
        }
    }
}
