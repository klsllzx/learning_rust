use std::cmp::Ordering;

use rand::Rng;
fn main() {
    let randnum = rand::thread_rng().gen_range(1..101);
    println!("rand numebr is {randnum}");
    

    loop {
        println!("guess a number");
    
        let mut guess =String::new();
        std::io::stdin().read_line(&mut guess).expect("fail to read the number");
        println!("the number you guess is {guess}");
        let guess:u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(err)=>{
                println!("{}",err);
                continue;

            }
        };
        match guess.cmp(&randnum){
            Ordering::Less=> println!("Less"),
            Ordering::Equal=>{
                println!("Equal");
                break;
            },
            Ordering::Greater=>println!("Greater")
        };
    }

}
//    export RUST_BACKTRACE=1
