use std::io::{self, Write};

struct CPU {
    memory: [u8; 4096],
}
// example 0 8 ENTER 0FC 108 ENTER
impl CPU {
    fn new() -> Self {
        CPU {
            memory: [0; 4096], // Initialize memory with zeros
        }
    }

    fn print_memory(&self, start: usize, end: usize) {
        // Limit the range to avoid going out of memory bounds
        let start = start.max(0).min(self.memory.len() - 1);
        let end = end.max(0).min(self.memory.len() - 1);

        for i in (start..=end).step_by(4) {
            // Format the address
            print!("0x{:03X}: ", i);
            // Print values in a line
            for j in 0..4 {
                if i + j <= end {
                    // Print value if it exists
                    print!("0x{:02X} ", self.memory[i + j]);
                } else {
                    // If the address is out of bounds, print a space
                    print!("      ");
                }
            }
            println!(); // Move to a new line after printing four values
        }
    }
}

fn main() {
    let mut cpu = CPU::new();

    // Initialize memory
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;    //<1>
    mem[0x002] = 0x21; mem[0x003] = 0x00;    //<2>
    mem[0x004] = 0x00; mem[0x005] = 0x00;    //<3>

    mem[0x100] = 0x80; mem[0x101] = 0x14;    //<4>
    mem[0x102] = 0x80; mem[0x103] = 0x14;    //<5>
    mem[0x104] = 0x00; mem[0x105] = 0xEE; 

    loop {
        // Prompt the user to input a memory range or 'exit'
        let mut input = String::new();
        print!("Enter memory range (start and end addresses separated by space) or 'exit' to quit: ");
        io::stdout().flush().unwrap(); // Flush the output buffer

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // Check if the user wants to exit
        if input.eq_ignore_ascii_case("exit") {
            break; // Exit the loop
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.len() != 2 {
            println!("Error: you must enter two addresses.");
            continue;
        }

        // Convert the entered strings to numbers
        let start: usize = match usize::from_str_radix(parts[0], 16) {
            Ok(num) => num,
            Err(_) => {
                println!("Error: invalid starting address.");
                continue;
            }
        };

        let end: usize = match usize::from_str_radix(parts[1], 16) {
            Ok(num) => num,
            Err(_) => {
                println!("Error: invalid ending address.");
                continue;
            }
        };

        // Print memory values in the specified range
        cpu.print_memory(start, end);
        
        println!("========================================="); // Print a line for separating outputs
    }
}
