fn main() {
    println!("Hello, world!");
    struct User{
        name:String,
        passwd:u32,
    };
    let mut a = User{
        name:String::from("lzx"),
        passwd:123
    };
    let t="lzx";
    let b=&t[1..];
    a.name=(*b).to_string();
    struct Color(i32,i32);
    let black =Color(0,0);
}
