pub mod ar;
use sibyl_core::Series;

pub trait Config {}

pub trait Model {
    type Config: Config;
    type S: Series + ?Sized;

    fn new(config: Self::Config) -> Self
    where
        Self: Sized;
    fn fit(&mut self, series: &Self::S);
    fn predict(&self, series: &Self::S, steps: usize) -> Self::S;
}
