use std::io::stdin as inp;

/// Напишите программу, которая определяет минимальное число в последовательности 
/// положительных чисел, которую ввел пользователь. Если в последовательности есть отрицательные числа, 
/// то вы должны сообщить об этом пользователю и предложить повторить ввод еще раз.

pub fn min_digit(){
    let mut input = String::new();
    let result = inp().read_line(&mut input);
    let mut min_dig = 0;
    match result {
        Ok(_) =>{
            let arr:Vec<&str> = input.split_whitespace().collect();
            //println!("{:?}", arr);
            let res = arr.iter().try_for_each(|str|{
                let rr = str.chars().any(|c|{
                    c.is_numeric()
                });
                if rr == false { Err(())}else{ Ok(())}
            });

            match res {
                Ok(_) =>{
                    let arr1:Vec<i32> = arr.into_iter()
                    .map(|it|{
                        it.parse::<i32>().unwrap()
                    })
                    .collect();
                    println!("{:?}", arr1);
                    let res1 = arr1.iter().enumerate().try_for_each(|(ind, it)|{
                        return if *it > 0 {
                            if ind == 0 {min_dig = *it};
                            if *it < min_dig {min_dig = *it};
                            Ok(())
                        }else{
                            Err(())
                        };                        
                    });
                    if let Err(_) = res1 {
                        println!("Negative number. Try again");
                        return;
                    } 
                    //println!("{min_dig}");
                }
                Err(_) =>{println!("Not numeric"); return}
            }
            
        },
        Err(e) => println!("{e}")
    }
    println!("{min_dig}");
}