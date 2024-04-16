fn main() {
    let mut s =String::from("Hello, world!");
    let index=get_first_word(&s);
    let hello=&s[..5]

}

fn get_first_word(s:&String)->usize{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return i;
        }
    }
    s.len()
}
//&str
//never have a[1..0]