#[derive(Debug)]
pub struct User {
    name: String,
    age: isize,
}

impl User {
    pub fn from(name: String, age: isize) -> User {
        User {
            name: name,
            age: age,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    pub fn res(&self) -> Result<isize, std::io::Error> {
        return super::userstore::user::res();
    }

    pub fn resi(&self) -> Result<isize, std::io::Error> {
        let i = super::userstore::user::res()?; // res方法的返回值是Result类型，并且resi方法也必须是返回Result类型的时候，才能使用?来传递错误
        Ok(i)
    }
}
