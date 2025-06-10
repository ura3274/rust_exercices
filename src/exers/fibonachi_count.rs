use std::time::Instant;

fn fiba(value:i32)->i32 {
    let mut i:i32 = 2;
	while (i*i)<= value {
		if value%i == 0 {
			return 0;
		}
        i+=1; 
	}
	return 1;
}

pub fn fiba_count() {
    let mut list = vec![0;2000000];
    let tm = Instant::now();
    //let mut m = 0;
	let mut j = 0;
	for m in 2..100000000 {
		if list[1999999] != 0 {
			break;
		}
		if fiba(m) == 1 {
			list[j] = m;
			j+=1;
		}
	}
    println!("Elapsed:{:?}", tm.elapsed());
    
}