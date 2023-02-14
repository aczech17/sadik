// USAGE:
// program [-a [file names] [archive name]] | [-d [archive name]]


pub(crate) enum ProgramOptions
{
    Archive {file_names: Vec<String>, archive_filename: String},
    Unarchive {archive_filename: String},
    Invalid,
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
        let archive_name = &args[args_len - 1];
        let file_names = &args[2..args_len - 1];

        return ProgramOptions::Archive
        {
            file_names: file_names.to_vec(),
            archive_filename: archive_name.to_string(),
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