use std::{fs::File, io::{BufRead, BufReader, Write}};

/// Напишите программу учета оценок студентов. Для этого создайте текстовый файл с именем 
/// students.txt, содержащий список из 10 студентов и их оценки по трем 
/// предметам: математика, физика и информатика.
///
/// Содержимое файла:
///
///   в первой строке находится общее количество студентов;
///
///   в каждой последующей строке находится ФИО студента и три целых числа (оценки);
///
///   данные в строке разделены пробелами, а оценки варьируются в диапазоне от 1 до 5.
///
/// Затем создайте класс, с помощью которого вы будете считывать данные из файла. На экран выведите ФИО студентов с оценками в порядке убывания их среднего балла.

pub fn students(){
    //if !Path::exists(Path::new("gggg.txt")){
    //let mut readStr = String::new();
    let str = String::from(
r#"Steve Peter Lucas 5 5 5
Peter Peter Smith 4 3 3
Igor Nico State 5 2 2
Tom Rawell Sawyer 4 3 5 
Ura Alex Buza 5 5 5"#);
        let file_res = File::create("students.txt");
        match file_res {
            Ok(mut f) => {
                match f.write_all(str.as_bytes()) {
                    Ok(_) => println!("Ok write to file"),
                    Err(e) => println!("Error write to file: {:?}", e)
                }
            },
            Err(e) => println!("{:?}", e)
        }

        match File::open("students.txt") {
            Ok(f) => {
                let mut map_stud:Vec<(f32, String)> = Vec::new();
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    match line {
                        Ok(s) => {
                            //println!("{:?}", s);
                            let mut acc = 0f32;
                            
                            s.chars().all(|c|{
                                //println!("{c}");
                                if c.is_numeric() {
                                    let num = c.to_string().parse::<f32>().unwrap_or_default();
                                    acc += num;
                                    println!("{acc}");
                                    true
                                }else {true}
                            });

                            map_stud.push((acc/3f32, s));
                            
                        },
                        Err(e) => println!("Failed read file: {:?}",e)
                    }
                }
                
                map_stud.sort_by(|(key1, _), (key2, _)|{
                    //let kk1 = key1.parse::<f32>().unwrap_or_default();
                    //let kk2 = key2.parse::<f32>().unwrap_or_default();
                    key1.total_cmp(&key2)
                });
                for (key,value) in map_stud {
                    println!("key: {:?}, value: {:?}", key, value);
                }
                
            },
            Err(e) => println!("Fail open file: {:?}", e)
        }


    //}
    
}