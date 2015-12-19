fn main() {
	is_empty();
    len();
    swap();
}

fn is_empty() {
    let a = [1, 2, 3];
    println!("is a empty slice? {}", a.is_empty());
}



fn len(){
    let a = [1, 2, 3];
    println!("the number of elements in the slice is {}", a.len());
	
}

fn swap(){
	 let mut a = [1, 2, 3];
	 a.swap(0,1);
    println!("after swap {:?}",a );
}
