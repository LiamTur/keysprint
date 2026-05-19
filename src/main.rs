use std::time::{Duration, SystemTime};
//use std::thread::sleep;
use std::io::{self, stdout, Write};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size},
    cursor::{MoveTo,MoveLeft,MoveRight,MoveDown, SavePosition, RestorePosition},
    style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute},
};
use scopeguard::defer;
use rand::prelude::*;

struct position {
    x: u16,
    y: u16
}


//      //   // //////  //      // ////////   ///////  ///////    //////  //     //   //////////
//      //  //  //        //  //   //         //    // //    //     //    ////   //       //
//      ////    ////        //     ///////    ///////  ///////      //    //  // //       //
//      // //   //          //          //    //       //   //      //    //   ////       //
//      //  //  //////      //    ////////    //       //    //   //////  //     //       //

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    let (width, height) = size()?;
    let position = position {
        x: width,
        y: height,
    };    

    execute!(stdout(), MoveTo(position.x / 4, position.y / 4))?;
    execute!(stdout(), MoveRight(10));
    print!("//   // //////  //      // ////////   ///////  ///////    //////  //     //   //////////");
    execute!(stdout(), MoveDown(1));
    execute!(stdout(), MoveLeft(88));
    print!("//  //  //        //  //   //         //    // //    //     //    ////   //       //");
    execute!(stdout(), MoveDown(1));
    execute!(stdout(), MoveLeft(84));
    print!("////    ////        //     ///////    ///////  ///////      //    //  // //       //");
    execute!(stdout(), MoveDown(1));
    execute!(stdout(), MoveLeft(84));
    print!("// //   //          //          //    //       //   //      //    //   ////       //");
    execute!(stdout(), MoveDown(1));
    execute!(stdout(), MoveLeft(84));
    print!("//  //  //////      //    ////////    //       //    //   //////  //     //       //");
    execute!(stdout(), MoveDown(2));
    execute!(stdout(), MoveLeft(84));




    let words = ["test", "fine", "method", "string", "vote", "fire", "guest", "mutation", "laser", "truncate", "hobby", "impulse", "reinforce", "motorist", "spit", "scene", "warm", "relinquish", "owe", "realism", "channel", "extinct", "ankel", "punish", "wait", "abolish", "progressive", "begin", "foster"];

    let amount_of_words: usize = 15;

    let mut overflow_letters: usize = 0;

    let mut score: usize = 0;
    fn choose_random_word(words: &[&str]) -> Vec<char>{
        let lenght: i32 = (words.len()) as i32;
        let mut rng = rand::rng();
        let mut nums: Vec<i32> = (0..lenght).collect();
        nums.shuffle(&mut rng);
        let random: usize = nums
            .choose(&mut rng)
            .map(|&n| n as usize)
            .unwrap();
        words[random].chars().collect()
    }

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


    fn timeupdate(width: u16, height: u16, now: SystemTime){
        execute!(stdout(), SavePosition);
        execute!(stdout(), MoveTo(width/2, height));


        match now.elapsed() {
        Ok(elapsed) => {
            let time = elapsed.as_secs_f64();
            print!("time: {:.2}", time);
            stdout().flush();
        }
        Err(e) => {
            print!("what? {e:?}");
            stdout().flush();
        }
    }

        execute!(stdout(), RestorePosition);
    }
    
    let mut displayed_w: Vec<Vec<char>> = Vec::new();

    println!("write the words shown, or Ctrl+c to escape, to start press any button:");
    execute!(stdout(), MoveLeft(80));

    execute!(stdout(), SavePosition);
    for i in 0..amount_of_words+1 {
        displayed_w.push(choose_random_word(&words));
        let word: String = displayed_w[i].iter().collect();
        print!("{} ", word);
    }


    let mut to_be = 0;

    let mut test: Vec<char> = displayed_w[to_be].clone();

    execute!(stdout(), RestorePosition);
    execute!(stdout(), MoveDown(4));

    defer! {
        let _ = disable_raw_mode();
    }

    loop {
        let event = read()?;

        match event {
            Event::Key(KeyEvent { .. }) => {
                break;
            }
            _ => {}
        }
    }
    let now = SystemTime::now();


    loop {
        timeupdate(position.x, position.y, now);
        if to_be == amount_of_words+1 {
            print!("You are done! ");
            stdout().flush()?;
            break;
        }
        if event::poll(std::time::Duration::from_millis(50))? {
            match read()? {
                Event::Key(key) => {
                    // Quit on Ctrl+C
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        println!("\nCtrl+C pressed. Quitting...");
                        stdout().flush()?;
                        break;
                    }
                    if  key.code == KeyCode::Backspace {
                        execute!(stdout(), MoveLeft(1))?;
                        execute!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();
                        if overflow_letters <= 0 {
                            overflow_letters = 0;
                            if score == 0{
                                execute!(stdout(), ResetColor);
                                score = 0;
                            }else{
                                score -= 1;
                            }
                        }else {
                            if overflow_letters == 1{
                                execute!(stdout(), ResetColor);
                            }
                            overflow_letters -= 1;
                        }
                    }
                    if score == test.len() && key.code == KeyCode::Char(' ')
                    {
                        execute!(stdout(), ResetColor);
                        to_be += 1;
                        stdout().flush()?;
                        if to_be == amount_of_words+1 {
                            print!(" You are done! ");
                            stdout().flush()?;
                            break;
                        }
                        test = displayed_w[to_be].clone();
                        score = 0;
                        execute!(stdout(), MoveRight(1))?;
                    }
                    if score < test.len() 
                    {
                    if key.code == KeyCode::Char(test[score]) && overflow_letters == 0
                    {
                        print_key_event(key);
                        stdout().flush()?;
                        if score >= test.len() {
                            execute!(stdout(),SetForegroundColor(Color::Red));
                            overflow_letters += 1;
                        }else{
                            score += 1;
                        }
                    }else {
                        if key.code != KeyCode::Backspace && key.code != KeyCode::Char(' ') {
                            overflow_letters += 1;
                           execute!(stdout(),SetForegroundColor(Color::Red));
                            print_key_event(key);
                            stdout().flush()?;
                        }
                    }
                    }else if key.code != KeyCode::Backspace
                    {
                        execute!(stdout(),SetForegroundColor(Color::Red));
                        overflow_letters += 1;
                        print_key_event(key);
                        stdout().flush()?;
                    }
                }
                _ => {}
            }
        }
    }
 
    match now.elapsed() {
        Ok(elapsed) => {
            let time = elapsed.as_secs_f64();
            let wpm = (amount_of_words as f64) / (time / 60.0);

            println!("wpm: {:.2}", wpm);
        }
        Err(e) => {
            println!("what? {e:?}");
        }
    }

    disable_raw_mode()?;
    Ok(())
}
