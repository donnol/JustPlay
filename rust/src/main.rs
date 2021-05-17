#![allow(unused)]

use std::fmt::Debug;
use std::ops::Add;

mod usersrv; // 还要在这里声明mod的文件路径，好难理解
use usersrv::user::User; // 文件路径::模块名::结构体名

use usersrv::userstore::user;

fn main() {
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
