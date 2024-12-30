
use super::get_default_music_directory;
use crate::cli::list;
use std::path::Path;
use std::io::Write;
use std::fs;
use std::path::PathBuf;

#[test]
fn get_default_msc_directory() {
    // -- FIXTURE
    let msc_dir = get_default_music_directory().unwrap();
    // -- CHECK
    assert_eq!(msc_dir.to_str(), Some("src/_dev/msc"));
}
#[test]
fn get_list() {
    // FIXTURE
    let msc_dir = "src/_dev/msc/hero's hour/";

    let music_directory_path = fs::canonicalize("./")
        .unwrap()
        .join(PathBuf::from(msc_dir));

    let music_directory = fs::read_dir(music_directory_path.clone())
        .unwrap();
    let mut writer = std::io::BufWriter::new(std::io::stdout()); 
    let x = list(&mut writer, music_directory, &music_directory_path);

    println!("{:?}", x);

}
