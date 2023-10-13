pub trait Dump {
    fn dump(&self);
}

impl<T: std::fmt::Debug> Dump for T {
    fn dump(&self) {
        println!("{:?}", self);
    }
}

