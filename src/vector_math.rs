// assume that vector to vector operations are performed with vectors of the same size
pub fn add_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + v2[i]).collect()
}
pub fn add_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + scalar).collect()
}

pub fn subtract_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - v2[i]).collect()
}
pub fn subtract_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - scalar).collect()
}

pub fn multiply_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x * scalar).collect()
}

pub fn divide_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x / scalar).collect()
}

pub fn magnitude(v1: &Vec<f32>) -> f32 {
    v1.into_iter().map(|x| x.powi(2)).fold(0., |sum, x| sum + x).sqrt()
}

pub fn distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    (0..v1.len())
        .map(|i| (v1[i] - v2[i]).powi(2))
        .fold(0., |sum, x| sum + x)
        .sqrt()
}

pub fn midpoint(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len())
        .map(|i| (v1[i] + v2[i]) / 2.)
        .collect()
}
