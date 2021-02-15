use chrono::{Datelike, Utc};
use std::fs::OpenOptions;
use std::io::Write;

pub fn add_line(text: &str) -> std::io::Result<()> {
    let now = Utc::now();
    let mut file = OpenOptions::new().create(true).append(true).open(format!(
        "daily-{:04}-{:02}-{:02}",
        now.year(),
        now.month(),
        now.day()
    ))?;
    file.write_all(text.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}
