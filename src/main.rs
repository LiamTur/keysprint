use std::time::{Duration, SystemTime};
//use std::thread::sleep;
use std::io::{self, stdout, Write};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size},
    cursor::MoveTo,
};
use scopeguard::defer;
use rand::prelude::*;

struct position {
    x: u16,
    y: u16
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    let (width, height) = size()?;
    let position = position {
        x: width,
        y: height,
    };    
    execute!(stdout(), MoveTo(position.x / 4, position.y / 4))?;
    let now = SystemTime::now();

    let words = ["test", "fine", "method", "string", "vote", "fire", "guest"];

    let lenght: i32 = (words.len() - 1) as i32;

    let mut rng = rand::rng();

    let mut nums: Vec<i32> = (0..lenght).collect();
    nums.shuffle(&mut rng);

    let random: usize = nums
        .choose(&mut rng)
        .map(|&n| n as usize)
        .unwrap();

    
    let test: Vec<char> = words[random].chars().collect();
    let mut score: usize = 0;
    println!("write {}, or x to escape:", String::from_iter(test.clone()));


    fn print_key_event(key: KeyEvent) {
    let modifiers = match key.modifiers {
        KeyModifiers::NONE => "None",
        KeyModifiers::SHIFT => "Shift",
        KeyModifiers::CONTROL => "Ctrl",
        KeyModifiers::ALT => "Alt",
        _ => "Multiple",
    };
 
    match key.code {
        KeyCode::Char(c) => print!("{}", c),
        code => print!("{:?}", code),
        }
        stdout().flush();
    }
    defer! {
        let _ = disable_raw_mode();
    }
    loop {
        if score == test.len()
        {
            print!("You are done!");
            stdout().flush()?;
            break;
        }
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    // Quit on Ctrl+C
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        println!("\nCtrl+C pressed. Quitting...");
                        stdout().flush()?;
                        break;
                    }
                    print_key_event(key);
                    if key.code == KeyCode::Char(test[score]) {
                        stdout().flush()?;
                        score += 1;
                    }
                }
                _ => {}
            }
        }
    }
 
    match now.elapsed() {
        Ok(elapsed) => {
            let time = elapsed.as_secs_f64();
            let wpm = (test.len() as f64 / 5.0) / (time / 60.0);

            println!("wpm: {:.2}", wpm);
            //let mut wpm = score/elapsed.as_mins();
            //println!("wpm: ", wpm);
        }
        Err(e) => {
            println!("what? {e:?}");
        }
    }

    disable_raw_mode()?;
    Ok(())
}
