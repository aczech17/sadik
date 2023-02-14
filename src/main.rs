use crate::file_info::FileInfoVector;

mod file_info;
mod archive;

use crate::archive::archive_files;

fn main()
{
    const USAGE: &str = "super archiwizator i dearchiwizator kurwo";

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        eprintln!("{USAGE}");
        return;
    }

    let file_info_vector = FileInfoVector::new(&args[1..]);
    archive_files(file_info_vector, "out").unwrap();
}
