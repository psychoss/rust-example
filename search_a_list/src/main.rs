fn main(){
    let haystack=vec!["B","A","B","E","F","A"];

    println!("{}",haystack.iter().position(|s| *s=="A").unwrap());
    println!("{}",haystack.iter().rposition(|s| *s=="A").unwrap());
    
}