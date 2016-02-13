use std::env;
use std::fs;
use std::fs::File;
use std::io;

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
fn parse(container: &str, mut archive: zip::read::ZipArchive<File>) {
    info!("start");
    for i in 0..(archive.len()) {
        let mut file = archive.by_index(i).unwrap();
        if file.name().ends_with(".class") {
            println!("{}\t{}", container, file.name().replace("/", "."));
        } else if file.name().ends_with(".jar") {
            info!("jar file found: {}", file.name());
            let unpacked_jar = unpack(&mut file);
            parse(file.name(), zip::ZipArchive::new(unpacked_jar).unwrap());
        }
    }
}

fn unpack(file: &mut zip::read::ZipFile) -> File {
    let mut out_path = env::temp_dir();
    out_path.push("rust-jar-walker");
    out_path.push(sanitize_file_name(file.name()));
    create_directory(out_path.parent().unwrap_or(std::path::Path::new("")));
    write_file(file, &out_path);
    return File::open(&out_path).unwrap();
}

fn create_directory(outpath: &std::path::Path) {
    fs::create_dir_all(&outpath).unwrap();
}

fn write_file(file: &mut zip::read::ZipFile, out_path: &std::path::Path) -> File {
    let mut out_file = File::create(&out_path).unwrap();
    io::copy(file, &mut out_file).unwrap();
    return out_file;
}

fn sanitize_file_name(file_name: &str) -> std::path::PathBuf {
    let no_null_file_name = match file_name.find('\0') {
        Some(index) => &file_name[0..index],
        None => file_name,
    };

    std::path::Path::new(no_null_file_name)
        .components()
        .filter(|component| *component != std::path::Component::ParentDir)
        .fold(std::path::PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let file_name = std::path::Path::new(&args[1]);
            let root_file = File::open(&file_name).unwrap();
            let zip_archive = zip::ZipArchive::new(root_file).unwrap();
            parse(&*args[1], zip_archive);
        },
        _ => {
            help();
        }
    }
}
