use std::path::Path;
use std::fs::{self, DirEntry};

fn main() {
    let path = Path::new("/home/psycho/Downloads");
    if fs::metadata(&path).unwrap().is_dir() {
        for entry in fs::read_dir(&path).unwrap() {
            let file = entry.unwrap();
            if file.file_type().unwrap().is_file() {
                match file.path().extension() {
                    Some(expr) => {
                        let x = expr.to_str().unwrap();
                        if x == "zip" {
                            println!("{:?}{:?}", file.path(), x);
                        }
                    }
                    _ => {}
                }
                // if  file.path().extension().unwrap().to_str()==".zip"{

            }
            // }
        }
    }

}
