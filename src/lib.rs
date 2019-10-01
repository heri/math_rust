
#[macro_use]
extern crate helix;

pub fn h_distance(lat1: &f64, lng1: &f64, lat2: &f64, lng2: &f64 -> usize {
    let earth_radius_kilometer = 6371.0_f64;
    
    let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34880_f64);
    let (london_latitude_degrees, london_longitude_degrees) = (51.50853_f64, -0.12574_f64);

    let lat1_r = lat1.to_radians();
    let lat2_r = lat2.to_radians();

    let delta_latitude = (lat1 - lat2).to_radians();
    let delta_longitude = (lng1 - lng2).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + lat1_r.cos() * lat2_r.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    return distance;
}

ruby! {
    class MathRust {
        def haversine_distance(lat1: Float, lng1: Float, lat1: Float, lng1: Float)) -> Float {
            return h_distance(&lat1, &lng1, &lat2, &lng2);
        }
    }
}