use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut filename = "sandmark.umz".to_owned();
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        filename = args[1].clone();
    }
    println!("Loading program scroll {}", filename);
    let mut f = File::open(filename)?;
    
    let mut scroll = vec![0; 0];
    f.read_to_end(&mut scroll)?;

    println!("Loaded {} sandstone platters from program scroll", scroll.len()/4);
    
    println!("First platter {:?} {:08b} {:08b} {:08b} {:08b}", &scroll[0..4], scroll[0], scroll[1], scroll[2], scroll[3]);
    
    Ok(())
}
