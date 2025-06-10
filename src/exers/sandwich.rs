use std::io::stdin as inp;

/// Сэндвич с мороженым — это строка, образованная двумя одинаковыми концами и разной серединой. 
/// Например:

/// AABBBAA
/// 3&&3
/// yyyyymmmmmmmmyyyyy
/// hhhhhhhhmhhhhhhhh

/// Обратите внимание, что левый и правый концы сэндвича идентичны как по длине, 
/// так и по повторяющимся символам. Середину составляет третий (отличный от первых двух) набор символов.

/// Следующее не является сэндвичем с мороженным:

/// BBBBB // вы не можете иметь только мороженное (без сэндвича)
/// AAACCCAA // вы не можете иметь неравные по длине окончания в сэндвиче
/// AACDCAA // вы не можете иметь начинку из разных символов
/// A // ваш сэндвич не может быть менее трех символов

/// Напишите программу, которая возвращает true, если строка, введенная пользователем, 
/// является сэндвичем с мороженым, и false — в противном случае.

/// Примеры:

/// isIcecreamSandwich ("CDC") ➞ true
/// isIcecreamSandwich ("AAABB") ➞ false
/// isIcecreamSandwich ("AA") ➞ false

/// Примечание: Сэндвич с мороженым должен иметь минимальную длину 3 символа, и как минимум 2 из этих символов должны быть различны.


pub fn sandwich_game()->bool{
    let mut input = String::new();
    
    println!("Введите сэндвич:");
    let result = inp().read_line(&mut input);
    match result {
        Ok(_) => {
            if input.len()-2 < 3 {println!("less than 3"); return false}
            if input.chars().nth(0).unwrap() != input.chars().nth(input.len()-3).unwrap() {println!("first layer fault"); return false};
            let y = input.len()-3;
            let mut layer = 1;
            for i in 1..=y{
                if i > y-i {
                    if layer == 2{
                        println!("Good"); return true;
                    } else {
                        println!("Bad"); return false;
                    } 
                }else if i == y-i {
                    if input.chars().nth(i) != input.chars().nth(i-1){
                        layer += 1;
                    }
                }else {
                    if input.chars().nth(i).unwrap() == input.chars().nth(i-1).unwrap(){
                        if input.chars().nth(i).unwrap() != input.chars().nth(y-i).unwrap() {println!("fault"); return false};
                    }else {
                        if input.chars().nth(i).unwrap() != input.chars().nth(y-i).unwrap() {println!("second fault"); return false};
                        layer += 1;
                    }
                }
                if layer == 3 {println!("To much layer"); return false}
            }
        },
        Err(e) => println!("{e}")
    };
        false
    }