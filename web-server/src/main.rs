extern crate hyper;
use std::io::prelude::*;
use std::io;
use std::env;
use std::fs::File;


use hyper::{Get, Post};
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;

macro_rules! wout { ($($tt:tt)*) => { {writeln!($($tt)*)}.unwrap() } }

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
);

fn hook(mut req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref path) => match (&req.method, &path[..]) {
            (&Get, "/") | (&Get, "/echo") => {

                static_file("index.html", res);
                // try_return!(res.send(b"Try POST /echo"));
                return;
            }
            (&Post, "/echo") => (), // fall through, fighting mutable borrows
            _ => {

                println!("{:?}", &path[..]);
                *res.status_mut() = hyper::NotFound;
                return;
            }
        },
        _ => {
            *res.status_mut() = hyper::NotFound;
            return;
        }
    }
}



fn main() {
    // .unwrap().handle(index);
    match run("127.0.0.1:8080") {
        Ok(()) => {
            println!("{:?}", "The server start at 127.0.0.1:8080 ");
        }
        Err(error) => {
            let mut eout = io::stderr();
            wout!(eout, "ERROR: {}", error);

        }
    }
}
fn run(address: &str) -> Result<(), hyper::error::Error> {
    let x = try!(Server::http(address));
    x.handle(hook);
    Ok(())
}

fn static_file(uri: &'static str, mut res: Response) -> Result<(), io::Error> {
    let mut path = try!(env::current_dir());
    let file_name = path.as_path().join("static").join(uri);
    let mut file = try!(File::open(file_name));
    let mut datas: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut datas));
    res.send(&datas);
    // path.as_path().join("/staic");
    // path.push(uri);
    // println!("{:?}", file_name);
    Ok(())
}
