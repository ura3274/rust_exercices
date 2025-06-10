/// Напишите программу, которая проверяет, является ли введенная пользователем строка целым числом.
///
/// Пример результата выполнения программы:
///
/// Введите строку: 36.7
/// 36.7 не является целым числом

pub fn is_integer(){
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let mut not_int = false;
            let g = input.chars().all(|c|{
                if c.is_alphabetic() {
                    return false;
                } else if c.is_ascii_punctuation() {
                    if c == '.' {
                        not_int = true;
                    }else{
                        return false;
                    }  
                }
                true
            });
            if !g {println!("Not number");} else if not_int {println!("Not integer");} else {println!("Is integer");}

        },
        Err(e) => println!("{:?}", e)
    }
}