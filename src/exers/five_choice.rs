/// Сыграйте с компьютером в игру, используя следующие 5 вариантов наборов чисел:
///
/// Набор №1: 6, 7, 8
///
/// Набор №2: 7, 8, 9
///
/// Набор №3: 6, 9, 10
///
/// Набор №4: 6, 9, 8
///
/// Набор №5: 7, 6, 10
///
/// Введите с клавиатуры свой вариант набора чисел (из вышеприведенных) и сравните с набором 
/// чисел компьютера, который выбирается рандомно из 5 вышеприведенных наборов. 
/// Если сумма цифр вашего набора чисел больше суммы цифр набора чисел компьютера, 
/// то вы выиграли (и наоборот). 
/// В случае одинаковых сумм цифр — ничья.

pub fn five_choice(){
    let arr = [
        [6,7,8],
        [7,8,9],
        [6,9,10],
        [6,9,8],
        [7,6,10]
    ];
    let mut input = String::new();
    let num = rand::random_range(0..(arr.len()-1));
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let my_arr:Vec<i32> = input.split_whitespace()
            .map(|s|{
                s.parse::<i32>().unwrap()
            })
            .collect();

            let sum = arr[num].into_iter().reduce(|a, s|{
                a+s
            }).unwrap();
            let my_sum = my_arr.into_iter().reduce(|a, s|{
                a+s
            }).unwrap();

            if sum < my_sum {
                println!("You win: {sum} < {my_sum}");
            }else if sum > my_sum {
                println!("You lose: {sum} > {my_sum}");
            }else {println!("nichia: {sum} == {my_sum}");}
        },
        Err(e) => println!("{:?}", e)
    }
}