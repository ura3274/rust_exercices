use std::io::stdin as inp;

/// Напишите программу, которая вычисляет сумму первых n целых положительных четных чисел. 
/// Количество суммируемых чисел вводит пользователь.
/// Пример результата выполнения программы:
/// Введите количество суммируемых чисел: 12
/// Сума первых 12 целых положительных четных чисел равна 156

pub fn sum_numbers(){
    let mut input = String::new();
    let result = inp().read_line(&mut input);
    input = input.trim_end().to_string();
    match result {
        Ok(_) =>{
            let num = input.parse::<i32>().unwrap_or_else(|_|{
                12
            });
            let sum = (0..=num).into_iter().filter(|el|{
                el%2 == 0
            }).reduce(|a, s|->i32{
                a+s
            }).unwrap();
            println!("{sum}")
        },
        Err(e) => println!("{e}")
    }
}