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

    fn fit(&mut self, _series: &Self::S) {
        self.trained = true;
        todo!();
    }

    fn predict(&self, series: &Self::S, steps: usize) -> Self::S {
        if !self.trained {
            panic!("Model must be trained before prediction.");
        }
        let mut prediction = Vec::with_capacity(steps);
        let data = series.as_slice();
        for s in 0..steps {
            for i in 0..self.coefficients.len() {
                let idx = data.len().saturating_sub(i + 1);
                let value = if idx < data.len() { data[idx] } else { 0.0 };
                let coeff = self.coefficients.get(i).cloned().unwrap_or(0.0);
                if prediction.len() <= i {
                    prediction.push(0.0);
                }
                prediction[s] += coeff * value;
            }
            
        }
        Self::S::from(prediction)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sibyl_core::Series;

    #[test]
    #[should_panic(expected = "Model must be trained before prediction.")]
    fn test_ar_predict_before_training() {
        let config = AutoRegressiveConfig { order: 2 };
        let mut model = AutoRegressiveModel::<Vec<f64>>::new(config);
        let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let prediction = model.predict(&series, 1);
    }
}