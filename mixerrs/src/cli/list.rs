use crate::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn list(
    mut writer: impl std::io::Write,
    music_directory: fs::ReadDir,
    music_directory_path: &PathBuf,
) -> Result<(), Error> {
    writeln!(writer, "listing files in: {:?}", music_directory_path);
    for directory in music_directory {

        let sub_directory_name = directory.unwrap().file_name();
        writeln!(writer, "Name: {:?}", sub_directory_name);

        let sub_dir = fs::read_dir(music_directory_path.join(sub_directory_name))?;
        for file in sub_dir.into_iter() {
            writeln!(writer, "{:?}", file?.file_name());
        }
    }

    Ok(())
}
