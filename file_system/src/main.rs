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
                            let new_path = path.join(x);
                    

                            if !new_path.exists() {

                                match fs::create_dir(&new_path) {
                                    Ok(()) => {
                                        println!("ok");
                                    }
                                    Err(err) => {
                                        println!("Error");
                                        return;
                                    }
                                }
                            }
                            let new_file_name=new_path.join(file.path().file_name().unwrap().to_str().unwrap());
                            fs::rename(file.path(),&new_file_name);
                            
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
