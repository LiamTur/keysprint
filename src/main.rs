use std::time::{Duration, SystemTime};
//use std::thread::sleep;
use std::io;

fn main() {
    println!("Hello, world!");
    let now = SystemTime::now();
    
    let test: [char; 4] = ['f','i','n','e']; 
    println!("write 'fine', or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        let mut t = true;
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        let chars: Vec<char> = input_string.trim().chars().collect();
        println!("You wrote {}", input_string);
        for i in 0..4 {
            if test[i] != chars[i]{
                t = false;
            }
        if t {
            println!("correct!");
            }else{
                println!("that wasn't what I asked for!");
            }
        }
    }

    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("what? {e:?}");
        }
    }
}
