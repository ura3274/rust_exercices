use std::{collections::HashMap, io::stdin as inp, time::Duration};
use rodio::{OutputStream, Sink, source::SineWave, Source};

/// Напишите программу-телеграф, которая принимает от пользователя сообщение 
/// и выводит его на экран в виде последовательности точек и тире. 
/// Вывод точек и тире можно сопровождать звуковым сигналом соответствующей длительности.
 
pub fn morze(){
  let map_morze:HashMap<char, &str> = HashMap::from([
    ('А', "·−"),('Б',"−···"),('В',"·−−"),('Г',"−−·"),
    ('Д',"−··"),('Е',"·"),('Ж',"···−"),('З',"−−··"),
    ('И',"··"),('Й',"·−−−"),('К',"−·−"),('Л',"·−··"),
    ('М',"−−"),('Н',"−·"),('О',"−−−"),('П',"·−−·"),
    ('Р',"·−·"),('С',"···"),('Т',"−"),('У',"··−"),
    ('Ф',"··−·"),('Х',"····"),('Ц',"−·−·"),('Ч',"−−−·"),
    ('Ш',"−−−−"),('Щ',"−−·−"),('Ъ',"·−−·−·"),('Ы',"−·−−"),
    ('Ь',"−··−"),('Э',"··−··"),('Ю',"··−−"),('Я',"·−·−")
  ]);

    let mut input = String::new();
    let result = inp().read_line(&mut input);
    input = input.trim_end().to_string().to_uppercase();
    let (_stream, output_stream) = OutputStream::try_default().unwrap();
    //println!("{:?}", input);
    match result {

        Ok(_) => {
            input.chars().all(|c|{
                if c.is_alphabetic() {
                    //println!("{}", &c);
                    let sym = map_morze.get(&c).unwrap_or_else(||{&"err"});
                    println!("{sym}");
                    true
                }else{
                    println!(" ");
                    let beep = SineWave::new(800f32).take_duration(Duration::from_millis(500));
                    let sink = Sink::try_new(&output_stream).unwrap();
                    sink.append(beep);
                    
                    sink.sleep_until_end();
                    
                    true
                }
            });

            /*for i in 0..input.chars().count() {
                let c = input.chars().nth(i).unwrap();
                if  c.is_alphabetic() {
                    let sym = map_morze.get(&c).unwrap_or_else(||{&"err"});
                    println!("{sym}");
                }else {
                    println!(" ");
                    let beep = SineWave::new(440f32).take_duration(Duration::from_millis(500));
                    let sink = Sink::try_new(&output_stream).unwrap();
                    sink.append(beep);
                    sink.sleep_until_end();
                }
            }*/
        },
        Err(e) => println!("{e}")
        
    }


}