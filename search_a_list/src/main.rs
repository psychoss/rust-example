fn main() {
    let haystack = vec!["A", "B", "E", "F", "A"];
    let mut h_ite = haystack.iter();
    println!("{}", h_ite.position(|s| *s == "A").unwrap());
    println!("{}", h_ite.rposition(|s| *s == "A").unwrap());

}
