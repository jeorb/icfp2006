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
    
    //let mut platters = vec![0; 0
    let mut arrays: Vec<Vec<u32>> = Vec::new();
    let mut registers: [u32; 8] = [0; 8];
    let mut a0: Vec<u32> = Vec::new();
    
    let len = scroll.len();
    let mut i = 0;
    while i < len {
        let platter: u32
            = ((scroll[i] as u32) << 24)
            + ((scroll[i+1] as u32) << 16)
            + ((scroll[i+2] as u32) << 8)
            + (scroll[i+3] as u32)
        ;
        a0.push(platter);
        i+=4;
    }
    arrays.push(a0);
    
    println!("First platter {} {:032b} {} {}", arrays[0][0], arrays[0][0], arrays.len(), arrays[0].len());

    let mut finger = 0;
    while finger < arrays[0].len() {
        let platter = arrays[0][finger];
        let operator = platter >> 28;
        let a = (platter & 7) as usize;
        let b = (platter >> 3 & 7) as usize;
        let c = (platter >> 6 & 7) as usize;
        println!("Operator {} {:04b} - {} {} {}", operator, operator, a, b, c);
        match operator {
            0 => {
                println!("Conditional Move");
                if registers[c] != 0 {
                    registers[a] = registers[b];
                }
            },
            1 => {
                println!("Array Index");
            },
            2 => {
                println!("Array Amendment");
            },
            3 => {
                println!("Addition");
            },
            4 => {
                println!("Multiplication");
            },
            5 => {
                println!("Division");
            },
            6 => {
                println!("Not-And");
            },
            7 => {
                println!("Halt");
            },
            8 => {
                println!("Allocation");
            },
            9 => {
                println!("Abandonment");
            },
            10 => {
                println!("Output");
            },
            11 => {
                println!("Input");
            },
            12 => {
                println!("Load Program");
            },
            13 => {
                println!("Orthography");
            },
            _ => {
                println!("Whoops!!");
            }
            
        }
        finger+=1;
        if finger  > 10 {
            break;
        }
    }

    Ok(())
}
