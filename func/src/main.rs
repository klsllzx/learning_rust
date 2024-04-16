fn main() {
    println!("Hello, world!");
    an_function(5);
    control();
}
/*comment */
fn an_function(x:isize)->i32{//need type
    println!("an_function {}",x);
    let y = {
        let x=1;
        x+1
    };
    println!("{}",y);
    if y < 5{
        println!("<5");
    }
    else if y<1{
        println!("<1");
    }
    else {
        println!(">5");
    }
    match y<5{
        true=>println!("<5"),
        fail=>println!(">5")
    };
    y
}

fn control(){
    let mut num =0;
    let x= while num != 5{
        num+=1;
        if num==3{
            break;
            num;
        }
    };
    println!("{}",num);
    let mut cnt =0;
    let arr= [5;7];
    for i in arr.iter(){
        println!("{}",i);
    }
    for i in (1..4).rev(){
        println!("{}",i)
    }
}