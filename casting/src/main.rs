macro_rules! p{
($a:expr,$b:expr)=>{
println!(" {} => {:?}",$a,$b);
}
}
fn main() {


    p!("char a to u8", 'a' as u8);
    p!("u8 97 to char", 97 as char);
    p!("u8 97 to usize", 97 as usize);

    let mut s = String::new();

    s.push_str("Hello world");
    p!("String to Vec<u8>", String::into_bytes(s));
    p!("&'static str to Vec<u8>", "Hello world".as_bytes());



}
