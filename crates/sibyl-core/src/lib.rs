pub trait Series {
    fn as_slice(&self) -> &[f64];
    fn from(data: Vec<f64>) -> Self;
}

impl Series for Vec<f64> {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn from(data: Vec<f64>) -> Self {
        data
    }
}
