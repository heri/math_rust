
#[macro_use]
extern crate helix;

use std::collections::HashMap;
use std::error::Error;

pub fn h_distance(coord1: &[f64], coord2: &[f64]) -> f64 {
    if coord1.len() < 2 || coord2.len() < 2 {
        panic!("Both coordinates must have at least two elements.");
    }

    const EARTH_RADIUS_KM: f64 = 6371.0;

    let (lat1, lon1) = (coord1[0].to_radians(), coord1[1].to_radians());
    let (lat2, lon2) = (coord2[0].to_radians(), coord2[1].to_radians());

    let delta_lat = lat1 - lat2;
    let delta_lon = lon1 - lon2;

    let central_angle_inner = (delta_lat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);

    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    EARTH_RADIUS_KM * central_angle
}

pub fn variance_f32(data: &[f32], mean: f32) -> f32 {
    let len = data.len();
    if len <= 1 {
        return 0.0; // Variance undefined
    }

    let numerator: f32 = data.iter().map(|&x| (x - mean).powi(2)).sum();
    numerator / (len - 1) as f32
}

pub fn mean_f32(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    values.iter().sum::<f32>() / values.len() as f32
}

pub fn array_to_vec(arr: &[f32]) -> Vec<f32> {
     arr.iter().cloned().collect()
}

pub fn median_f32(vect: &[f32]) -> f32 {
    if vect.is_empty() {
        panic!("Cannot compute median of an empty array.");
    }

    let mut numbers = vect.to_vec();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;

    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    }
}

pub fn mode(vect: &[f32]) -> f32 {
    if vect.is_empty() {
        panic!("Cannot compute mode of an empty array.");
    }

    let mut occurrences = HashMap::new();

    for &value in vect {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Unexpected error computing mode")
}


pub fn covariance_f32(x_values: &[f32], y_values: &[f32]) -> f32 {
    if x_values.len() != y_values.len() {
        panic!("x_values and y_values must have equal lengths.");
    }

    if x_values.is_empty() {
        return 0.0;
    }

    let mean_x = mean_f32(x_values);
    let mean_y = mean_f32(y_values);

    x_values
        .iter()
        .zip(y_values.iter())
        .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
        .sum::<f32>()
        / x_values.len() as f32
}


pub struct LinearRegression {
    pub coefficient: Option<f32>,
    pub intercept: Option<f32>
}
 
impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression { coefficient: None, intercept: None }
    }

    pub fn fit(&mut self, x_values: &[f32], y_values: &[f32]) -> Result<(), Box<dyn Error>> {
        if x_values.len() != y_values.len() || x_values.is_empty() {
            return Err("Input vectors must have the same non-zero length".into());
        }

        let x_mean = mean_f32(x_values);
        let y_mean = mean_f32(y_values);

        let covariance = covariance_f32(x_values, y_values, x_mean, y_mean);
        let variance = variance_f32(x_values, x_mean);

        if variance == 0.0 {
            return Err("Variance of x_values is zero, cannot perform regression".into());
        }

        let b1 = covariance / variance;
        let b0 = y_mean - b1 * x_mean;

        self.coefficient = Some(b1);
        self.intercept = Some(b0);
        Ok(())   
    }   

    pub fn predict(&self, x : f32) -> Result<f32, Box<dyn Error>> {
        match (self.coefficient, self.intercept) {
            (Some(b1), Some(b0)) => Ok(b0 + b1 * x),
            _ => Err("Model has not been fitted yet. Call `fit` first.".into()),
        }
    }

    pub fn predict_list(&self, x_values: &[f32]) -> Result<Vec<f32>, Box<dyn Error>> {
        x_values.iter().map(|&x| self.predict(x)).collect()
    }

    pub fn evaluate(&self, x_test: &[f32], y_test: &[f32]) -> Result<f32, Box<dyn Error>> {
        if x_test.len() != y_test.len() {
            return Err("Test vectors must have the same length".into());
        }

        let predictions = self.predict_list(x_test)?;
        root_mean_squared_error(y_test, &predictions)
    }
}

pub fn root_mean_squared_error(actual: &[f32], predicted: &[f32]) -> Result<f32, Box<dyn Error>> {
    if actual.len() != predicted.len() {
        return Err("Actual and predicted vectors must have the same length".into());
    }

    let mse = actual
        .iter()
        .zip(predicted.iter())
        .map(|(&a, &p)| (a - p).powi(2))
        .sum::<f32>()
        / actual.len() as f32;

    Ok(mse.sqrt())
}

pub fn standard_deviation_f32(data: &Vec<f32>, mean: f32) -> f32 {
    let var = variance_f32(data, &mean);
    let std_dev = var.sqrt();
    std_dev
}

pub fn max_f32(data: &Vec<f32>) -> f32 {
    let mut result = data[0];
    
    for item in data {
        if *item > result {
            result = *item;
        }
    }
    result
}

pub fn min_f32(data: &Vec<f32>) -> f32 {
    let mut result = data[0];
    
    for item in data {
        if *item < result {
            result = *item;
        }
    }
    result
}

pub fn max_usize(data: &Vec<usize>) -> usize {
    let mut result = data[0];
    
    for item in data {
        if *item > result {
            result = *item;
        }
    }
    result
}

pub fn min_usize(data: &Vec<usize>) -> usize {
    let mut result = data[0];
    
    for item in data {
        if *item < result {
            result = *item;
        }
    }
    result
}

ruby! {
    class LittleMath {
        def haversine_distance(coord1: Vec<Float>, coord2: Vec<Float>) -> Float {
            return h_distance(&coord1, &coord2);
        }

        def mean(array: Vec<Float>) -> Float {
            return mean_f32(&array);
        }

        // same as mean
        def average(array: Vec<Float>) -> Float {
            return mean_f32(&array);
        }

        def variance(array: Vec<Float>, mean: Float) -> Float {
            return variance_f32(&array, &mean);
        }

        def covariance(array1: Vec<Float>, array2: Vec<Float>) -> Float {
            return variance_f32(&array1, &array2);
        }

        // currently this tries to fit x_Values and y_values with a simple linear regression and then uses model to predict for value
        def linear_reg(x_values: Vec<Float>, y_values: Vec<Float>) -> Result<Hash, String> {
            let mut model = LinearRegression::new();
            if let Err(e) = model.fit(&x_values, &y_values) {
                return Err(e.to_string());
            }

            Ok(hash! {
                "intercept" => model.intercept.unwrap(),
                "coefficient" => model.coefficient.unwrap()
            })
        }

        def predict(x_values: Vec<Float>, intercept: Float, coefficient: Float) -> Vec<Float> {
            x_values
                .iter()
                .map(|&x| intercept + coefficient * x)
                .collect()
        }

        def evaluate(x_values: Vec<Float>, y_values: Vec<Float>, intercept: Float, coefficient: Float) -> Result<Float, String> {
            let predicted: Vec<f32> = x_values
                .iter()
                .map(|&x| intercept + coefficient * x)
                .collect();

            root_mean_squared_error(&y_values, &predicted).map_err(|e| e.to_string())
        }

        def standard_deviation(array: Vec<Float>, mean: Float) -> Float {
            return standard_deviation_f32(&array, &mean);
        }

        def min(array: Vec<Float>) -> Float {
            return min_f32(&array);
        }

        def max(array: Vec<Float>) -> Float {
            return max_f32(&array);
        }

        def median(array: Vec<Float>) -> Float {
            return median_f32(&array);
        }

        def mode(array: Vec<Float>) -> Float {
            return mode_f32(&array);
        }
        
    }
}