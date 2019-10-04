
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


pub fn average_u32(data: &Vec<u32>) -> f32 {
    let mut sum = 0;

    for item in data {
        sum = sum + *item;
    }

    let avg = sum as f32 / data.len() as f32;
    avg
}

pub fn average_f32(data: &Vec<f32>) -> f32 {
    let mut sum = 0.0;

    for item in data {
        sum = sum + *item;
    }

    let denominator = data.len();
    sum = sum / denominator as f32;
    sum
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

        def average(array: Vec<Float>) -> Float {
            return average_f32(&array);
        }

        def variance(array: Vec<Float>, mean: Float) -> Float {
            return variance_f32(&array, &mean);
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