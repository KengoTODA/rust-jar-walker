use std::env;
use std::fs::File;

#[macro_use]
extern crate log;
extern crate zip;

/// print usage for help
fn help() {
    println!("usage
rust-jar-walker <file_name>
  walk contents in specified jar file, and print name of contained classes")
}

/// parse zip archive, and print content inside of it
fn parse(archive: &mut zip::read::ZipArchive<File>) {
    info!("start");
    for i in 0..(archive.len()) {
        let file = archive.by_index(i).unwrap();
        let file_name = file.name();
        if file_name.ends_with(".class") {
            println!("class: {}", file_name);
        } else if file_name.ends_with(".jar") {
            // TODO parse inside of this jar file
            println!("jar: {}", file_name);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let file_name = std::path::Path::new(&*args[1]);
            let root_file = File::open(&file_name).unwrap();
            let mut zip_archive = zip::ZipArchive::new(root_file).unwrap();
            parse(&mut zip_archive);
        },
        _ => {
            help();
        }
    }
}
