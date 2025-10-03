use num_traits::Float;

pub trait Series {
    type Elem: Float;

    fn as_slice(&self) -> &[Self::Elem];
    fn from(vec: Vec<Self::Elem>) -> Self
    where
        Self: Sized;
}

pub trait TimeSeries: Series {
    fn timestamps(&self) -> &[f64];
}

pub trait MultivariateSeries<S: Series> {
    fn n_series(&self) -> usize;
    fn series(&self, index: usize) -> Option<S>;
}

impl<T: Float> Series for Vec<T> {
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem] {
        self.as_ref()
    }

    fn from(vec: Vec<Self::Elem>) -> Self
    where
        Self: Sized,
    {
        vec
    }
}

