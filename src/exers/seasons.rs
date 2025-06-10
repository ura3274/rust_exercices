use std::io::stdin as inp;

/// Напишите программу, которая запрашивает у пользователя номер месяца и затем выводит 
/// соответствующее название времени года. В случае, если пользователь введет недопустимое число, 
/// программа должна вывести сообщение об ошибке.

/// Пример результата выполнения программы:

/// Введите номер месяца (число от 1 до 12): 12
/// Зима

pub fn seasons(){
    let mut input = String::new();
    println!("Введите номер месяца");
    let result = inp().read_line(&mut input);
    input = input.trim_end().to_string();
    match result {
        Ok(_) =>{
            if input.len() > 2 || input.len() == 0{
                println!("Введите коректный номер месяца");
                return;
            }else {
                let res = input.chars().any(|c|{
                    c.is_numeric()
                });
                if res {
                    let num = input.parse::<i32>().unwrap();
                    match num {
                        12 | 1 | 2 => println!("Зима"),
                        3 | 4 | 5 => println!("Весна"),
                        6 | 7 | 8 => println!("Лето"),
                        9 | 10 | 11 => println!("Осень"),
                        _ => println!("Введите коректный номер месяца")
                    };                    
                }else{
                    println!("Введите коректный номер месяца");
                    return;
                }
            }
        },
        Err(e) =>{println!("{e}")}
        
    }
}