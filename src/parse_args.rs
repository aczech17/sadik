// USAGE:
// program [-a [file names] [archive name]] | [-d [archive name]]



use std::fs::metadata;
use walkdir::WalkDir;

pub(crate) enum ProgramOptions
{
    Archive { pathnames: Vec<String>, filenames: Vec<String>, archive_filename: String},
    Unarchive {archive_filename: String},
    Invalid,
}

fn get_pathnames_and_filenames(args: &[String]) -> (Vec<String>, Vec<String>)
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

pub(crate) fn parse_options(args: Vec<String>) -> ProgramOptions
{
    let args_len = args.len();
    if args_len < 2
    {
        return ProgramOptions::Invalid;
    }

    if args[1] == String::from("-a")
    {
        let archive_filename = &args[args_len - 1];
        let input_filenames = &args[2..args_len - 1];

        let (pathnames, filenames) =
            get_pathnames_and_filenames(input_filenames);

        return ProgramOptions::Archive
        {
            pathnames,
            filenames,
            archive_filename: archive_filename.to_string(),
        }
    }

    if args[1] == String::from("-d")
    {
        let archive_filename = &args[2];
        return ProgramOptions::Unarchive
        {
            archive_filename: archive_filename.to_string(),
        }
    }

    ProgramOptions::Invalid
}