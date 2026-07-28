use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::fs;
use chrono::prelude::*;

fn main() ->io::Result<()> {


    let package = match env::args().nth(1) {

        Some(name) => name,
        None => {
        println!("Usage: pacpeek <package>");

        return Ok(()); 
        }
    };

    let paths = match fs::read_dir("/var/lib/pacman/local/") {
        Ok(paths) => paths,
        Err(_) => {
            println!("Nice try Windows user, or a non-pacman user");
            return Ok(());
        }
    };

    for entry in paths {
        let entry = entry.unwrap();
        let name = entry.file_name();

        if name.to_string_lossy().starts_with(&package) {

            let fullpath = entry.path().join("desc");

            let mut file = File::open(fullpath)?;

            let mut content: String = String::new();

            file.read_to_string(&mut content)?;

            trimming(&content);

            break;
        }
    }
    Ok(())
}

fn trimming(content: &str) {
    let mut lines = content.lines();

    println!("_________________________________");

    while let Some(line) = lines.next() {

        if line.starts_with('%') {

            let trimmed_line = line.trim_matches('%');
            let value = lines.next().unwrap_or("");
            

            if trimmed_line == "SIZE"{

                let bytes: u64 = value.parse().unwrap_or(0);
                let megabytes = bytes as f64 / 1048576.0;

                println!("{}: {:.1} {}", trimmed_line, megabytes, "MB");

            } else if trimmed_line == "BUILDDATE" || trimmed_line == "INSTALLDATE"{

                let timestamp: u64 = value.parse().unwrap_or(0);
                let datetime = DateTime::from_timestamp(timestamp as i64, 0).unwrap();
                let formatted = datetime.format("%Y-%m-%d").to_string();

                println!("{}: {}", trimmed_line, formatted);

            } else {

                println!("{}: {}", trimmed_line, value);
            }
        }
    }

    println!("_________________________________");

}
