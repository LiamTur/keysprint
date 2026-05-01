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
    println!("Hello, world!");
    let now = SystemTime::now();
    
    let test = ['f','i','n','e',' ','1'];
    let mut score: usize = 0;
    println!("write {}, or x to escape:", String::from_iter(test));


    fn print_key_event(key: KeyEvent) {
    let modifiers = match key.modifiers {
        KeyModifiers::NONE => "None",
        KeyModifiers::SHIFT => "Shift",
        KeyModifiers::CONTROL => "Ctrl",
        KeyModifiers::ALT => "Alt",
        _ => "Multiple",
    };
 
    match key.code {
        KeyCode::Char(c) => print!("Key: '{}' (Modifiers: {})", c, modifiers),
        code => print!("Key: {:?} (Modifiers: {})", code, modifiers),
        }
        stdout().flush();
    }
    defer! {
        let _ = disable_raw_mode();
    }
    loop {
        if score == 6
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
                        print!("that's correct!");
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
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("what? {e:?}");
        }
    }

    disable_raw_mode()?;
    Ok(())
}
