#[derive(Debug)]
struct Square{
    length:u32,
    width:u32,
}
fn main() {
    println!("Hello, world!");

    let a = Square{
        length:2,
        width:3,
    };
    let ret = area(&a);
    println!("{:?}",a);
    println!("{:#?}",a);
}

fn area(sq:&Square)->u32{
    let length=sq.length;
    let width=sq.width;
    let ret =length*width;
    ret

}