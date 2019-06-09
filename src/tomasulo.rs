mod common_data_bus;
mod instruction;
mod reserve_station;
mod platform;

use std::io;

fn main() {
    let mut t = vec![1, 2, 3];
    let mut context = platform::Platform::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break
                }
                match input.split_whitespace().next() {
                    Some(x) => context.load_inst(&x.to_string()),
                    None => Ok(()),
                };
            }
            Err(error) => break,
        };
    }
    while(!context.step()) {
        //thread::sleep_ms(500);
    }

    println!("=============================");
    
    context.print_inst_state();
}