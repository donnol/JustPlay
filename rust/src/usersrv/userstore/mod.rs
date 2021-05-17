pub mod user {
    pub fn store() {
        println!("store.")
    }

    use std::io;

    pub fn res() -> Result<isize, io::Error> {
        Ok(1)
    }
}
