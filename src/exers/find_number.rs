use std::io::stdin as inp;
//use rand;

/// Напишите программу, реализующую игру «Угадай число». Компьютер загадывает число от 0 до 999 
/// (используйте генерацию случайных чисел), а пользователь угадывает его. 
/// На каждом шаге угадывающий делает предположение, а задумавший число — сообщает, 
/// сколько цифр из числа угаданы и сколько из угаданных цифр занимают правильные позиции в числе. 
/// Например, если задумано число 725 и выдвинуто предположение, что задумано число 523, 
/// то угаданы две цифры (5 и 2), и одна из них занимает верную позицию. Например:

/// Компьютер загадал трехзначное число. Вы должны его отгадать. После очередного числа вам будет сообщено, сколько цифр угадано и сколько из них находится на своих местах.
/// Ваш вариант: 123
/// Угадано: 0. На своих местах: 0
/// Ваш вариант: 456
/// Угадано: 1. На своих местах: 1
/// Ваш вариант: 654
/// Угадано: 2. На своих местах: 2
/// Ваш вариант: 657
/// Угадано: 2. На своих местах: 2
/// Ваш вариант: 658
/// Угадано: 3. На своих местах: 3
/// ***Вы угадали число 658!***


pub fn find_number(){

    let mut input = String::new();
    let mut quite = true;
    let num = 789;//rand::random_range(0..=999);
    let mut num_alpha = num.to_string();
    let mut finded = 0;
    let mut finded_arr:Vec<char> = vec!['f'; 3];
    let mut in_place = 0;

    while quite {
        println!("Угадано {finded}. На своих местах {in_place}");
        println!("Введите число от 0 до 999");
        let result = inp().read_line(&mut input);
        input = input.trim_end().to_string();
        
        match result {
            Ok(_) =>{
                let res = input.chars().any(|c|{
                    c.is_numeric()
                });
                if res {
                    let my_num = input.parse::<i32>().unwrap();
                    match my_num {
                        0..=999 =>{
                            if my_num == num {
                                println!("Браво.Угадали {num}");
                                quite = false;
                            }else{
                                for i in 0..input.len() {
                                    for j in 0..num_alpha.len(){
                                       if input.chars().nth(i).unwrap() == num_alpha.chars().nth(j).unwrap() {
                                            if i == j {
                                                if finded_arr.contains(&input.chars().nth(i).unwrap()){
                                                    num_alpha = num_alpha.replace(num_alpha.chars().nth(j).unwrap(), "f");
                                                    finded_arr.remove(i);
                                                    in_place += 1;
                                                }else{
                                                    num_alpha = num_alpha.replace(num_alpha.chars().nth(j).unwrap(), "f");
                                                    finded += 1;
                                                    in_place += 1;
                                                }
                                                
                                            }else if !finded_arr.contains(&input.chars().nth(i).unwrap()) {
                                                finded_arr[i] = input.chars().nth(i).unwrap();
                                                finded += 1;
                                            } 
                                       } 
                                    }
                                } 
                            }
                        },
                        _ => println!("Введите число от 0 до 999")
                    }
                }else{
                    println!("Введите число от 0 до 999");
                    //quite = false;
                }
            },
            Err(e) => println!("{e}")
        }
        input.clear();
    }

    
}