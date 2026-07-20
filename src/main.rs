use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::fs;

fn main() ->io::Result<()> {


    let package = match env::args().nth(1) {

        Some(name) => name,

        None => {
        println!("Usage: cargo run <package>");

        return Ok(()); 
        }
    };

    let paths = fs::read_dir("/var/lib/pacman/local/").unwrap();

    for entry in paths {
        let entry = entry.unwrap();

        let name = entry.file_name();


        if name.to_string_lossy().starts_with(&package) {

            //println!("Found: {}", entry.path().display());

            //join
            let fullpath = entry.path().join("desc");

            //open
            let mut file = File::open(fullpath)?;

            //init string
            let mut content: String = String::new();

            //read
            file.read_to_string(&mut content)?;

            //print
            println!("{}", content);

            //kill that shi
            break;
        }
    }

    //println!("Searching for: {}", package);

    Ok(())
}
