use chrono::{Datelike, Utc};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub fn add_line(text: &str) -> std::io::Result<()> {
    let mut file = get_file()?;
    add_line_to_file(&mut file, text)
}
fn get_file() -> std::io::Result<File> {
    let now = Utc::now();
    OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(format!(
            "daily-{:04}-{:02}-{:02}",
            now.year(),
            now.month(),
            now.day()
        ))
}
pub fn get_daily_content() -> Option<String> {
    let file = get_file();
    match file {
        Ok(mut file_ok) => get_file_daily_content(&mut file_ok),
        Err(err) => {
            log::error!("Could not retrieve file {}", err);
            None
        }
    }
}

fn get_file_daily_content(file: &mut File) -> Option<String> {
    let mut contents = String::new();
    let last_position_result = file.seek(SeekFrom::Current(0));
    match last_position_result {
        Ok(last_position) => match file.seek(SeekFrom::Start(0)) {
            Ok(_start) => match file.read_to_string(&mut contents) {
                Ok(_bytes_read) => {
                    file.seek(SeekFrom::Start(last_position)).unwrap();
                    Some(contents)
                }
                Err(error) => {
                    log::error!("Could not read daily content {}", error);
                    None
                }
            },
            Err(error) => {
                log::error!(
                    "Could not read daily content. Impossible to move to initial position {}",
                    error
                );
                None
            }
        },
        Err(error) => {
            log::error!(
                "Could not read daily content. Impossible to retrieve current position {}",
                error
            );
            None
        }
    }
}

fn add_line_to_file(file: &mut File, text: &str) -> std::io::Result<()> {
    writeln!(file, "{}", text)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::daily::{add_line_to_file, get_file_daily_content};
    #[test]
    fn test_add_line() {
        let mut tmpfile = tempfile::tempfile().unwrap();
        add_line_to_file(&mut tmpfile, "plop").unwrap();

        // Read
        let file_content = get_file_daily_content(&mut tmpfile);
        assert_eq!(Option::Some("plop\n".to_string()), file_content);

        add_line_to_file(&mut tmpfile, "new_message").unwrap();

        let file_content = get_file_daily_content(&mut tmpfile);
        assert_eq!(
            Option::Some("plop\nnew_message\n".to_string()),
            file_content
        );
    }
}
