macro_rules! expr{
($arg:expr) => (3*$arg);
}

macro_rules! ident{
($arg:ident)=> (let $arg=10);
}

macro_rules! expr_multi{
($arg:expr)=>{
println!("{}",$arg);
println!("{}",$arg*3);
}
}
fn main() {
    println!("{}", expr!(5));
    println!("{}", expr!(2 + 3));
    ident!(x);
    println!("{}", x);
    expr_multi!(5);

}
