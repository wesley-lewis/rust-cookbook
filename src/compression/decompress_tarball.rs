use std::fs::File;
use flate2::write::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Archive;

pub fn run() {
    create_tarball_from_dir(); 
}

fn extract() {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path).expect("Expected a tar.gz file");

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    match archive.unpack(".") {
        Err(val) => println!("{}", val),
        _ => println!("No error"),
    }

}

fn create_tarball_from_dir() {
    let tar_gz = File::create("archive.tar.gz").expect("file couldn't be  created");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    match tar.append_dir_all("backup/", ".") {
        Err(err) => println!("err: {}", err),
        _  => println!("No error"),
    }
    
}

