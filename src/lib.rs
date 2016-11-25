use std::cmp;

// A hyperdimensional ball that may have other balls as children
#[derive(Debug, Clone)]
pub struct Ball {
    pub center: Vec<f32>,
    pub radius: f32,
    pub parent: Option<Box<Ball>>,
    pub left_child: Option<Box<Ball>>,
    pub right_child: Option<Box<Ball>>
}

fn main() {
    let b1 = Ball::new(vec![1., 3., 6., 3.]);
    println!("{:?}", b1);
    
    let b2 = Ball::new(vec![3., 5., 8., 5.]);
    println!("{:?}", b2);
    
    let pb = b1.bounding_ball(&b2);
    println!("{:?}", pb);
}

impl Ball {
    fn new(features: Vec<f32>) -> Ball {
        Ball {
            center: features, radius: 0.,
            parent: None, left_child: None, right_child: None
        }
    }

    fn bounding_ball(&self, other: &Ball) -> Ball {
        let span = subtract_vec(&self.center, &other.center);
        let mag = magnitude(&span);
        let unit_vec = divide_scal(&span, &mag);
        let p1 = add_vec(&self.center, &multiply_scal(&unit_vec, &self.radius));
        let p2 = subtract_vec(&other.center, &multiply_scal(&unit_vec, &other.radius));

        Ball {
            center: midpoint(&p1, &p2),
            radius: distance(&p1, &p2) / 2.,
            parent: None, left_child: None, right_child: None
        }
    }

    fn insert(&mut self, new_ball: Ball, is_left: bool) {
        let dist = distance(&self.center, &new_ball.center);
        // if outside of the ball, make a new parent ball and put both inside
        if dist > self.radius {
            match self.parent {
                Some(ref mut old_parent) => {
                    let mut new_parent = self.bounding_ball(&new_ball);
                    if is_left {
                        old_parent.left_child = Some(Box::new(new_parent));
                    } else {
                        old_parent.right_child = Some(Box::new(new_parent));
                    };
                    new_parent.parent = Some(*old_parent);
                    new_parent.right_child = Some(Box::new(*self));
                    new_parent.left_child = Some(Box::new(new_ball));
                    self.parent = Some(Box::new(new_parent));
                    new_ball.parent = Some(Box::new(new_parent));
                },
                
                None => {
                    // Must be the first ball in the tree.
                    // this reference needs to stay the same for the tree to function, so instead we
                    // inject a copy, and grow this ball to be the new parent.
                    let new_self_blueprint = self.bounding_ball(&new_ball);
                    self.left_child = Some(Box::new(*self));
                    self.right_child = Some(Box::new(new_ball));
                    self.center = new_self_blueprint.center;
                    self.radius = new_self_blueprint.radius;
                }
            }
        }
    }

    // fn search_similar(&self, features: &Vec<f32>, limit: usize) {

    // }
}


// pub struct BallTree {
//     first_ball: Option<&'a mut Ball<'a>>,
//     size: usize
// }

// impl BallTree {
//     fn new() -> BallTree {
//         BallTree {
//             first_ball: None,
//             size: 0
//         }
//     }

//     fn insert(&mut self, features: &Vec<f32>) {
//         match self.first_ball {
//             Some(ref ball) => {
//                 let mut ball = ball.borrow_mut();
//                 ball.insert(&Ball::new(features), false);
//             },
//             None => self.first_ball = Some(Ball::new(features))
//         }
//         self.size += 1;
//     }

//     // fn search_for_similar(&self, features: &Vec<f32>, limit: usize) -> Vec<Ball> {
//     //     match self.first_ball {
//     //         Some(first_ball) => {
//     //             first_ball.borrow()
//     //             .search(&features, limit)
//     //             .into_iter(|ball: Rc<RefCell<Ball>>| ball.clone().borrow_ref())
//     //         },
//     //         None => 
//     //     },
//     //     self.first_ball.search(&features, limit)
//     // }
// }



// useful vector functions
// assume that vector to vector operations are performed with vectors of the same size
fn add_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + v2[i]).collect()
}
fn add_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + scalar).collect()
}

fn subtract_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - v2[i]).collect()
}
fn subtract_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - scalar).collect()
}

fn multiply_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x * scalar).collect()
}

fn divide_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x / scalar).collect()
}

fn magnitude(v1: &Vec<f32>) -> f32 {
    v1.into_iter().map(|x| x.powi(2)).fold(0., |sum, x| sum + x).sqrt()
}

fn distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    (0..v1.len())
    .map(|i| (v1[i] - v2[i]).powi(2))
    .fold(0., |sum, x| sum + x)
    .sqrt()
}

fn midpoint(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len())
    .map(|i| (v1[i] + v2[i]) / 2.)
    .collect()
}
// yermyuurstd@om3-27@
