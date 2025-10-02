pub mod prelude;
pub mod forcasting;


pub trait Series {
    fn as_slice(&self) -> &[f64];
}

impl Series for Vec<f64> {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }
}
