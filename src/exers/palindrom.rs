/// Пользователь вводит натуральное четырехзначное число. 
/// Выясните, является ли оно палиндромом (читается одинаково как слева направо, так и справа налево).
///
/// Пример результата выполнения программы:
///
/// Введите число: 4884
/// 4884 является палиндромом

pub fn palindrom(){
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim_end().to_string();
            if input.len() == 4 {
                match input.parse::<i32>() {
                    Ok(_) => {
                        for i in 0..2 {
                            if input.chars().nth(i) != input.chars().nth_back(i){
                                println!("Is not palidrom");
                                return;
                            }
                            if i == 1 {println!("Is palidrom");}
                            
                        }
                    },
                    Err(_) => println!("Must be a number")
                }
            }else{
                println!("Must be 4");
                return;
            }
        },
        Err(e) => println!("{:?}", e)
    }
}