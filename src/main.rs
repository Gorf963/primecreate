//pub(crate) const SIZE: usize = 1000000 ;
pub(crate) const SIZE: usize = 100000000;

use std::time::Instant;
use std::thread;

fn main() {
    let builder = thread::Builder::new().name("largeStack".into()).stack_size(2*1024*1024*1024);
    let overall_start = Instant::now();
    let handler = builder.spawn(move || {
        find_prime();    
    }).unwrap();
    handler.join().unwrap();
    println!("{:?}", overall_start.elapsed());

}
fn find_prime() {

    let mut p: [u32;SIZE]=[0;SIZE];
    let mut count: usize = 3;
    let mut start = Instant::now();
    p[0]=2 as u32;
    p[1]=3 as u32;
    p[2]=5 as u32;
    p[3]=7 as u32;
let process_duration_start = start.elapsed();

start = Instant::now();
let mut process_duration_prime = start.elapsed();
let mut process_duration_backward = start.elapsed();
let mut process_duration_count = start.elapsed();
let mut process_duration_array = start.elapsed();

    for index in {11..SIZE as u32}.step_by(2) {
/*        
        let digits: Vec<_> = index.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        let mut u = 0 as u32;
        for s in digits.clone() {
            u += s;
        }
        if u%3==0 || u%9==0 {continue;}
        let c= digits.len();
        //check for 4 and 8
        u =&digits[c-1]+&digits[c-2]*10;
        if u%4==0 {continue;}
*/
        let mut prime = true;
process_duration_prime = start.elapsed();
let stop_index = (index as f64).sqrt() as u32 + 1;
start = Instant::now();
        for i in {0..count} {
            
            if index%p[i] as u32==0 {
                prime = false;
                break;
            }
            if stop_index<p[i] {break;}
        }
process_duration_backward = start.elapsed();

start = Instant::now();
        if prime {
            count +=1;
process_duration_count = start.elapsed();

start = Instant::now();
            p[count] = index;
process_duration_array = start.elapsed();

start = Instant::now();
            
        }
    }
//    process_duration = start.elapsed();
    /*
    for i in {0..count} {
        println!("{}",p[i]);
    }
    */
//    println!("{:?}",process_duration);
    println!("NUmber of primes {}", count+1);
    println!("start loop {:?}",process_duration_start);
    println!("Prime set {:?}",process_duration_prime);
    println!("backward check {:?}",process_duration_backward);
    println!("count increase {:?}",process_duration_count);
    println!("Append Array {:?}",process_duration_array);
}
