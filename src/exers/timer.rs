use std::{thread, time::{Duration}};
use rodio::Source;
use trpl;

/// Напишите программу-таймер, которая по истечении заданного промежутка времени 
///(который вводит пользователь) выдает звуковой сигнал.

pub fn timer(){
    
    let thr = thread::spawn(move||{
        //let tt = Instant::now();
        let (tx, mut rx) = trpl::channel();
        let ass = async{
            let (_output, output_handle) = rodio::OutputStream::try_default().unwrap();
            let syn = rodio::source::SineWave::new(600f32).take_duration(Duration::from_millis(500));
            let sink = rodio::Sink::try_new(&output_handle).unwrap();
            if rx.recv().await.unwrap(){
           //thread::sleep(Duration::from_millis(2000));
                sink.append(syn);
                sink.sleep_until_end();
            }
            
        };
        let ass1 = async{
            let mut count = 1;
            while count < 11{
                trpl::sleep(Duration::from_millis(1000)).await;
                println!("{count}");
                count += 1;
            }
            
            tx.send(true).unwrap();
        };
        trpl::run(
            trpl::join(ass1, ass)
        );
        //println!("{:?}", tt.elapsed());
    });
    let _ = thr.join();
    
}