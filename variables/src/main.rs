
const MAX_POINT:i32=32;//must typed
fn main() {
    let x = "   ";
    let x =5.0;
    let x = x+1.0;
    let x = x * 2.0;
    println!("Hello, world!");
    let guess :usize="42".parse().expect("error");
//i u 8 16 32 64 128
    let texts:usize=0b111_00;
    let val:u8 = 255;
    let emoji:char='\u{1F600}';
    let emoji='😄';
    let tup=(emoji,val,texts);
    let (x,y,z)=tup;
    let z= tup.0;
    println!("{z}");
    let a = [1,2,3,4,5,6,7];//fix
    
    let ab =[7;5];
    // let c=a[7];
    let c = a[ab[1]];
    //thread 'main' panicked at src/main.rs:23:13:
    //list is contiguously stored


}
