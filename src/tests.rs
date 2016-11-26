use super::*;

fn test_vec() -> Vec<f32> {
    vec![1., 2., 3., 4.]
}

#[test]
fn add_vec_test() {
    let test = add_vec(&test_vec(), &test_vec());
    assert_eq!(test, vec![2., 4., 6., 8.]);
}

#[test]
fn add_scal_test() {
    let test = add_scal(&test_vec(), 42);
    assert_eq!(test, vec![43., 44., 45., 46.]);
}

#[test]
fn subtract_vec_test() {
    unimplemented!();
}

#[test]
fn subtract_scal_test() {
    unimplemented!();
}

#[test]
fn multiply_scal_test() {
    unimplemented!();
}

#[test]
fn divide_scal_test() {
    unimplemented!();
}

#[test]
fn magnitude_test() {
    unimplemented!();
}

#[test]
fn distance_test() {
    unimplemented!();
}

#[test]
fn midpoint_test() {
    unimplemented!();
}

#[test]
fn new_ball_tree_test() {

}

#[test]
fn push_ball_tree_test() {
    
}
