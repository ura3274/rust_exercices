use std::io::{Write};
use crossterm::event::{read, Event};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::stdin as inp;

/// Напишите программу, которая вычисляет среднее арифметическое последовательности дробных чисел, 
/// вводимых с клавиатуры. После ввода пользователем последнего числа программа должна 
/// вывести минимальное и максимальное числа из последовательности. 
/// Количество чисел последовательности вводит пользователь.
///
/// Пример результата выполнения программы:
///
/// Введите количество чисел последовательности: 5
/// Введите последовательность: 5.4 7.8 3.0 1.5 2.3
/// Среднее арифметическое: 4.00
/// Минимальное число: 1.5
/// Максимальное число: 7.8

pub fn medium_sequence(){
    
    let mut arr:Vec<f32> = Vec::new();
    let mut count = 0;
    let mut num_str = String::new();
    let mut num_quant = 0;
    let mut input = String::new();

    println!("Input number quantity: ");

    let res_input = inp().read_line(&mut input);
    input = input.trim_end().to_string();

    match res_input {
        Ok(_) => {
            if input.chars().all(|c|{c.is_numeric()}) {
                num_quant = input.parse::<i32>().unwrap_or_else(|_|{0});
                let _ = enable_raw_mode();
                    loop{
                        if let Event::Key(event) = read().unwrap(){
                            if event.is_press() {
                                let code = event.code.to_string();
                                if code.chars().all(|c|{c.is_numeric()}) {
                                    print!("{code}");
                                    let _ = std::io::stdout().flush();
                                    num_str.push_str(&code);
                                }else if code == "Space"{
                                    count += 1;
                                    print!(" ");
                                    let _ = std::io::stdout().flush();
                                    let res = num_str.parse::<f32>();
                                    num_str.clear();
                                    match res {
                                        Ok(num) => {
                                            arr.push(num);
                                        },
                                        Err(e) => println!("{:?}", e)
                                    }
                                }else if code == "." {
                                    print!(".");
                                    let _ = std::io::stdout().flush();
                                    num_str.push_str(&code);
                                }
                                if count == num_quant {break;}
                                if code == "c" {break;}
                            } 
                        }  
                    }
                let _ = disable_raw_mode();
            }
        },
        Err(e) => println!("{:?}", e)
    }
    if !arr.is_empty() {
        let mut min = arr[0];
        let mut max = arr[0]; 
        let mut acc = 0f32;
        for i in &arr{
            if *i < min {min = *i}
            if *i > max {max = *i}
            acc += *i;
        }
        print!("\n");
        println!("Mean: {}", (acc/(arr.len() as f32)));
        println!("Minimum: {:?}", min);
        println!("Maximum: {:?}", max);
    }
    

    
    
}