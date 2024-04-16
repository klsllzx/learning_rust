fn main() {
    println!("Hello, world!");
    let s1=String::from("hello");
    let s2=s1;//move s1 is invalid not like python shallow copy, move
    let s3=s2.clone();//deepcopy
    println!("{}",s2);
    let strs=give_ownership();
    take_ownership(strs);
    // println!("{}",strs);
}

fn give_ownership()->String{
    let mut strs=String::from("hello");
    strs.push_str("fdsaf");
    strs
}
fn take_ownership(strs:String){

}
//one mut ref
//multio ref
