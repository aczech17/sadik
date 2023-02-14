use std::fs::File;
use std::io::Read;
use crate::file_info::{FileInfo, FileInfoVector};

mod file_info;
mod archive;

use crate::archive::archive_files;

fn main()
{
    /*
    let mut content: String = String::new();
    let mut file = File::open("out").unwrap();
    file.read_to_string(&mut content).unwrap();

    //println!("{content}");
    let s = format!("{}]", content.split("]").collect::<Vec<&str>>()[0]);

    let file_info_vector: FileInfoVector = serde_json::from_str(&s).unwrap();

    for file_info in file_info_vector.0
    {
        println!("{} {}", file_info.get_name(), file_info.get_size());
    }
    */

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
