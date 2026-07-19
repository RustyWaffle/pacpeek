use std::env;
use std::io::{self, Read};
use std::fs::File;

fn main() ->io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    for argument in args.iter(){
        println!("Searching for: {}", argument)
    }

    //open
    let mut file = File::open("/var/lib/pacman/local/firefox-152.0.6-1/desc")?;

    //init string
    let mut content: String = String::new();

    //read
    file.read_to_string(&mut content)?;

    println!("{}", content);
    
    Ok(())
}
