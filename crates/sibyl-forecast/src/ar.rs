use sibyl_core::Series;

use super::{Config, Model};

pub struct AutoRegressiveConfig {
    pub order: usize,
}

impl Default for AutoRegressiveConfig {
    fn default() -> Self {
        Self { order: 1 }
    }
}

impl Config for AutoRegressiveConfig {}

pub struct AutoRegressiveModel<S: Series> {
    // config: AutoRegressiveConfig,
    coefficients: Vec<f64>,
    _marker: std::marker::PhantomData<S>,
    trained: bool,
}

impl<S: Series> Model for AutoRegressiveModel<S> {
    type Config = AutoRegressiveConfig;
    type S = S;

    fn new(config: Self::Config) -> Self
    where
        Self: Sized,
    {
        Self {
            coefficients: Vec::with_capacity(config.order),
            // config,
            trained: false,
            _marker: std::marker::PhantomData,
        }
    }

    fn fit(&mut self, series: &Self::S) {
        todo!();
        self.trained = true;
    }

    fn predict(&self, series: &Self::S, steps: usize) -> Self::S {
        let predictions = series;
        todo!();
        // Vec::new()
    }
}
