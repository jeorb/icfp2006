use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::time::SystemTime;

const MAX: u64 = 1 << 32;

fn main() -> io::Result<()> {
    let start_time = SystemTime::now();
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
        let a = (platter >> 6 & 0b111) as usize;
        let b = (platter >> 3 & 0b111) as usize;
        let c = (platter & 0b111) as usize;
        match operator {
            0 => {
                //println!("Conditional Move");
                if registers[c] != 0 {
                    registers[a] = registers[b];
                }
            },
            1 => {
                //println!("Array Index");
                registers[a] = arrays[registers[b] as usize][registers[c] as usize];
            },
            2 => {
                //println!("Array Amendment");
                arrays[registers[a] as usize][registers[b] as usize] = registers[c];
            },
            3 => {
                //println!("Addition");
                registers[a] = ((registers[b] as u64) + (registers[c] as u64) % MAX) as u32;
            },
            4 => {
                //println!("Multiplication");
                registers[a] = ((registers[b] as u64 * registers[c] as u64) % MAX) as u32;
            },
            5 => {
                //println!("Division");
                registers[a] = registers[b] / registers[c];
            },
            6 => {
                //println!("Not-And");
                registers[a] = !(registers[b] & registers[c]);
            },
            7 => {
                //println!("Halt");
                break;
            },
            8 => {
                //println!("Allocation");
                arrays.push(vec![0; registers[c] as usize]);
                registers[b] = (arrays.len()-1) as u32;
            },
            9 => {
                //println!("Abandonment");
                arrays[registers[c] as usize] = vec![];
            },
            10 => {
                //println!("Output -------------------- {} - {}", registers[c] as u8 as char, registers[c]);
                print!("{}", registers[c] as u8 as char);
            },
            11 => {
                //println!("Input");
                let input: u8 = io::stdin().bytes().next().and_then(|result| result.ok()).unwrap();
                if input == '\n' as u8 {
                    registers[c] = !0;
                } else {
                  registers[c] = input as u32;
                }
            },
            12 => {
                //println!("Load Program");
                if registers[b] > 0 {
                    arrays[0] = arrays[registers[b] as usize].to_vec();
                }
                finger = registers[c] as usize;
                continue;
            },
            13 => {
                //println!("Orthography");
                let a = ((platter >> 25) & 0b111) as usize;
                registers[a] = platter & 0b11111111_11111111_11111111_1;
            },
            _ => {
                println!("Whoops!!");
                break;
            }
            
        }
        finger+=1;
    }

    println!("Completed processing in {} seconds.", start_time.elapsed().unwrap().as_secs());
    Ok(())
}
