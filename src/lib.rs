
#[macro_use]
extern crate helix;

pub fn h_distance(coord1: &Vec<f64>, coord2: &Vec<f64>) -> usize {
    let earth_radius_kilometer = 6371.0_f64;

    let lat1_r = coord1[0].to_radians();
    let lat2_r = coord2[0].to_radians();

    let delta_latitude = (coord1[0] - coord2[0]).to_radians();
    let delta_longitude = (coord1[1] - coord2[1]).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + lat1_r.cos() * lat2_r.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    return distance;
}

pub fn variance_f32(data: &Vec<f32>, mean: f32) -> f32 {
    let mut numerator = 0.0;

    for item in data {
        numerator = numerator + ((*item - mean) * (*item - mean));
    }

    let denominator = (data.len() - 1) as f32;
    let variance = numerator / denominator;
    variance
}

pub fn mean_f32(values : &Vec<f32>) -> f32 {
    if values.len() == 0 {
        return 0f32;
    }
 
    return values.iter().sum::<f32>() / (values.len() as f32);
}

pub fn covariance_f32(x_values : &Vec<f32>, y_values : &Vec<f32>) -> f32 {
    if x_values.len() != y_values.len() {
        panic!("x_values and y_values must be of equal length.");
    }
 
    let length : usize = x_values.len();
     
    if length == 0usize {
        return 0f32;
    }
 
    let mut covariance : f32 = 0f32;
    let mean_x = mean_f32(x_values);
    let mean_y = mean_f32(y_values);
 
    for i in 0..length {
        covariance += (x_values[i] - mean_x) * (y_values[i] - mean_y)
    }
 
    return covariance / length as f32;       
}

pub struct LinearRegression {
    pub coefficient: Option<f32>,
    pub intercept: Option<f32>
}
 
impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression { coefficient: None, intercept: None }
    }

    pub fn fit(&mut self, x_values : &Vec<f32>, y_values : &Vec<f32>) {
        let b1 = covariance_f32(x_values, y_values) / variance_f32(x_values);
        let b0 = mean_f32(y_values) - b1 * mean_f32(x_values);

        self.intercept = Some(b0);
        self.coefficient = Some(b1);       
    }   

    pub fn predict(&self, x : f32) -> f32 {
        if self.coefficient.is_none() || self.intercept.is_none() {
            panic!("fit(..) must be called first");
        }

        let b0 = self.intercept.unwrap();
        let b1 = self.coefficient.unwrap();

        return b0 + b1 * x;
    }

    pub fn predict_list(&self, x_values : &Vec<f32>) -> Vec<f32> {
        let mut predictions = Vec::new();

        for i in 0..x_values.len() {
            predictions.push(self.predict(x_values[i]));
        }

        return predictions;
    }

    pub fn evaluate(&self, x_test : &Vec<f32>, y_test: &Vec<f32>) -> f32 {
        if self.coefficient.is_none() || self.intercept.is_none() {
            panic!("fit(..) must be called first");
        }

        let y_predicted = self.predict_list(x_test);
        return self.root_mean_squared_error(y_test, &y_predicted);
    }

    fn root_mean_squared_error(&self, actual : &Vec<f32>, predicted : &Vec<f32>) -> f32 {
        let mut sum_error = 0f32;
        let length = actual.len();

        for i in 0..length {
            sum_error += f32::powf(predicted[i] - actual[i], 2f32);
        }

        let mean_error = sum_error / length as f32;
        return mean_error.sqrt();
    }
}

pub fn standard_deviation_f32(data: &Vec<f32>, mean: f32) -> f32 {
    let var = variance(data, mean);
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
    class MathRust {
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
        def linear_reg(x_values: Vec<Float>, y_values: Vec<Float>, value: Float) -> Float {
            let mut model = LinearRegression::new();
            model.fit(&x_values, &y_values);
            return model.predict(value);
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
        
    }
}