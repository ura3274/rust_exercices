use std::io::stdin as inp;

/// Напишите программу, которая объединяет два упорядоченных по возрастанию массива в один (тоже упорядоченный) массив.
///
/// Пример результата выполнения программы:
///
/// Введите элементы первого массива: 1 3 5 7 9
/// Введите элементы второго массива: 2 4 6 8 10
/// Массив-результат: 1 2 3 4 5 6 7 8 9 10

pub fn ordered_array(){
    let mut arr:Vec<i32> = Vec::new();
    let mut arr1:Vec<i32> = Vec::new();

    let mut input = String::new();
    let mut input1 = String::new();
    let res = inp().read_line(&mut input);
    let res1 = inp().read_line(&mut input1);

    match res {
        Ok(_) => {
            arr = input.split_whitespace().map(|el|{
                let rest1 = el.parse::<i32>();
                match rest1 {
                    Ok(num) => num,
                    Err(e) => {println!("{:?}", e); 0}
                }
            }).collect();
            //arr.sort();
            println!("{:?}", arr);
        },
        Err(e) => println!("{:?}", e)
    }

    match res1 {
        Ok(_) => {
            arr1 = input1.split_whitespace().map(|el|{
                let rest2 = el.parse::<i32>();
                match rest2 {
                    Ok(num) => num,
                    Err(e2) => {println!("{:?}", e2); 0}
                }
            }).collect();
            //arr1.sort();
            println!("{:?}", arr1);
        },
        Err(e1) => println!("{:?}", e1)
    }
    arr.append(&mut arr1);
    arr.sort();
    println!("{:?}", arr);
}