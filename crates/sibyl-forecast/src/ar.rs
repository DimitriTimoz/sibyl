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
    _coefficients: Vec<f64>,
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
            _coefficients: Vec::with_capacity(config.order),
            // config,
            trained: false,
            _marker: std::marker::PhantomData,
        }
    }

    fn fit(&mut self, _series: &Self::S) {
        self.trained = true;
        todo!();
    }

    fn predict(&self, series: &Self::S, _steps: usize) -> Self::S {
        let _predictions = series;
        todo!();
        // Vec::new()
    }
}
