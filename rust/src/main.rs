#![allow(unused)]

use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Add;
use std::path::PathBuf;
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use structopt::StructOpt;

mod usersrv; // 还要在这里声明mod的文件路径，好难理解
use usersrv::user::User; // 文件路径::模块名::结构体名

use usersrv::userstore::user;

mod store;
use store::store::test_mysql;

use std::error::Error;

use std::time;
use std::time::Duration;
use std::thread::sleep;

fn test_timer() {
    let interval = Duration::from_millis(1000);

    println!("wait");

    sleep(interval);

    println!("Done");

    for i in 1..10 {
        println!("wait in loop: {:?}", i);
        sleep(interval);
        println!("done in loop: {:?}", i);
    };
}

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // test();
    test_timer();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn test() {
    let r = add(1, 2);
    println!("r: {}", r);

    add_without_return(2, 3);

    let s = String::from("abc");
    let sr = str(&s[..]);
    println!("sr: {}", sr);
    let s = String::from("");
    let sr = str(&s[..]);
    println!("sr: {}", sr);

    let u = User::from(String::from("jd"), 19);
    println!("u: {:?}", u);

    user::store();

    let r1 = user::res();
    println!("r1: {:?}", r1.unwrap());

    let r2 = u.res();
    println!("r2: {:?}", r2.unwrap());

    let r3 = u.resi();
    println!("r3: {:?}", r3.unwrap());

    let opt = Opt::from_args();
    println!("{:#?}", opt);

    match opt {
        Opt::Add { debug, verbose } => {
            println!("{:#?}, {:#?}", debug, verbose)
        }
        Opt::Get {
            speed,
            output,
            nb_cars,
            level,
            files,
        } => {
            println!(
                "{:#?}, {:#?}, {:#?}, {:#?}, {:#?}",
                speed, output, nb_cars, level, files
            );
            for file in files {
                println!("{:#?}", file);
                let fr = File::open(file);
                match fr {
                    Ok(mut f) => {
                        println!("file: {:#?}", f);
                        let data = f.metadata();
                        println!("data: {:#?}", data);

                        let mut buf = Vec::new();
                        f.read_to_end(&mut buf);
                        println!("buf: {:#?}", str::from_utf8(&buf))
                    }
                    Err(e) => {
                        println!("e: {:#?}", e)
                    }
                };
            }
        }
        _ => (),
    };

    let r = test_mysql();
    println!("test_mysql: {:#?}", r);

    // 多线程：通过channel传递数据
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    thread::spawn(move || tx.send(1));

    println!("revc: {:#?}", rx.recv());
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "rcli", about = "rust cli demo")]
enum Opt {
    #[structopt(name = "add")]
    Add {
        // A flag, true if used in the command line. Note doc comment will
        // be used for the help message of the flag. The name of the
        // argument will be, by default, based on the name of the field.
        /// Activate debug mode
        #[structopt(short, long)]
        debug: bool,

        // The number of occurrences of the `v/verbose` flag
        /// Verbose mode (-v, -vv, -vvv, etc.)
        #[structopt(short, long, parse(from_occurrences))]
        verbose: u8,
    },

    #[structopt(name = "get")]
    Get {
        /// Set speed
        #[structopt(short, long, default_value = "42")]
        speed: f64,

        /// Output file
        #[structopt(short, long, parse(from_os_str))]
        output: PathBuf,

        // the long option will be translated by default to kebab case,
        // i.e. `--nb-cars`.
        /// Number of cars
        #[structopt(short = "c", long)]
        nb_cars: Option<i32>,

        /// admin_level to consider
        #[structopt(short, long)]
        level: Vec<String>,

        /// Files to process
        #[structopt(name = "FILE", parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

fn add<T>(a: T, b: T) -> T
where
    T: Add + Add<Output = T>,
{
    return a + b;
}

fn add_without_return<T>(a: T, b: T)
where
    T: Add<Output = T> + Debug,
{
    let r = a + b;
    println!("r: {:?}", r);
}

fn str(s: &str) -> &str {
    if s.len() > 0 {
        return &s[..1];
    } else {
        return s;
    }
}
