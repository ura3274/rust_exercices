use std::path::Path;

mod exers;
//use std::time::Instant;
//use trpl;

fn main() {
    
    //let res = exers::sandwich::sandwich_game();
    //println!("{res}");
    //exers::min_digit::min_digit();
    //exers::seasons::seasons();
    //exers::find_number::find_number();
    //exers::sum_numbers::sum_numbers();
    //exers::morze::morze();
    //exers::medium_sequence::medium_sequence();
    //exers::ordered_array::ordered_array();
    //exers::timer::timer();
    //exers::students::students();
    //exers::is_integer::is_integer();
    //exers::clock::clock();
    //exers::palindrom::palindrom();
    exers::five_choice::five_choice();
    //let a = String::new();
    //let a = Box::new(A{a:3, b:"gg".to_string()});
    //let b = A{a:3, b:"jj".to_string()};
    //let c = &a;
    /*let start = Instant::now();
    const N:usize = 10000000;
    let range1 = 0..N;
    let range2 = 0..N;
    let rr:Vec<usize> = range1.zip(range2).map(|(a,b)|{
       a + b
    }).collect();
    //let mut rr = vec![0; N];
    //for i in 0..rr.len() {
      //  rr[i] = i + i;
    //}

    let duration = start.elapsed();
    println!("Время выполнения: {:.2?}", duration.as_millis());
    println!("{:?}", rr.get(N-1).unwrap());*/
    //let a = A{a:"str", b:0};
    /*println!("start");
    trpl::run(async{
      let g = gllop().await;
      println!("{g}");
    });
    let mut a = Box::new(2);
    *a = 3;
    let b = &mut a;
    let c = a;
    //println!("{}");
    println!("end");*/
    //let j = std::fs::create_dir(A{a:"hhh", b: 0f32});
    
}

struct A<'a, T>{
  a: &'a str,
  b:T
}

fn rt()->impl B<i32, Output = String>{
  A { a: "ggg", b: 0 }
}

trait B<T>{
  type Output;
  fn ab()->T;
}

impl<'a, T> B<i32> for A<'a, T>{
  type Output = String;
  fn ab()->i32{
      0
  }
}

impl <'a, T> AsRef<Path> for A<'a, T>{
  fn as_ref(&self) -> &Path {
      Path::new(self.a)
  }
}