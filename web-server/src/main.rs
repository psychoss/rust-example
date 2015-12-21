extern crate hyper;

use std::io::{self, Write};

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
                try_return!(res.send(b"Try POST /echo"));
                return;
            }
            (&Post, "/echo") => (), // fall through, fighting mutable borrows
            _ => {
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
