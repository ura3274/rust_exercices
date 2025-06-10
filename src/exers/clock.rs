use std::{io::Write, thread, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

/// Напишите программу, которая выводит на экран работающие «электронные часы», 
/// которые работают в течение, например, 3 минут или до тех пор, 
/// пока пользователь не нажмет любую клавишу.


pub fn clock(){
    
    let mut minute = 0;
    let mut second = 0;
    
    let _ = crossterm::terminal::enable_raw_mode();
        loop{
            if poll(Duration::from_millis(1)).unwrap(){
                if let Event::Key(ev) = read().unwrap(){
                    if ev.code == KeyCode::Char('1'){
                            break;
                        }
                }
                
            }else{
                print!("\r{:02}:{:02}", minute, second);
                            let _ = std::io::stdout().flush();
                            thread::sleep(Duration::from_millis(1000));
                            second += 1;
                            if second == 60 {
                                minute += 1;
                                second = 0;
                            }
            }  
        }
    let _ = crossterm::terminal::disable_raw_mode();
}