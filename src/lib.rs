use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

#[cfg(tests)]
mod tests;

// A hyperdimensional ball that may have other balls as children.
// should be created with the Ball::new()
#[derive(Clone)]
pub struct Ball {
    pub center: Vec<f32>,
    pub radius: f32,
    pub parent: Option<Rc<RefCell<Ball>>>,
    pub left_child: Option<Rc<RefCell<Ball>>>,
    pub right_child: Option<Rc<RefCell<Ball>>>
}

pub struct BallTree {
    first_ball: Option<Rc<RefCell<Ball>>>,
    size: usize
}

impl BallTree {
    fn new() -> BallTree {
        BallTree {
            first_ball: None,
            size: 0
        }
    }

    fn insert(&mut self, features: &Vec<f32>) {
        match self.first_ball {
            Some(ref ball) => {
                let mut ball = ball.borrow_mut();
                ball.insert(&Ball::new(features), false);
            },
            None => self.first_ball = Some(Ball::new(features))
        }
        self.size += 1;
    }

    // fn search_for_similar(&self, features: &Vec<f32>, limit: usize) -> Vec<Ball> {
    //     match self.first_ball {
    //         Some(first_ball) => {
    //             first_ball.borrow()
    //             .search(&features, limit)
    //             .into_iter(|ball: Rc<RefCell<Ball>>| ball.clone().borrow_ref())
    //         },
    //         None => 
    //     },
    //     self.first_ball.search(&features, limit)
    // }
}

impl Ball {
    fn new(features: &Vec<f32>) -> Rc<RefCell<Ball>> {
        Rc::new(RefCell::new(Ball {
            center: features.clone(), radius: 0.,
            parent: None, left_child: None, right_child: None
        }))
    }

    fn bounding_ball(&self, other: &Ball) -> Rc<RefCell<Ball>> {
        let span = subtract_vec(&self.center, &other.center);
        let mag = magnitude(&span);
        let unit_vec = divide_scal(&span, &mag);
        let p1 = add_vec(&self.center, &multiply_scal(&unit_vec, &self.radius));
        let p2 = subtract_vec(&other.center, &multiply_scal(&unit_vec, &other.radius));

        Rc::new(RefCell::new(Ball {
            center: midpoint(&p1, &p2),
            radius: distance(&p1, &p2) / 2.,
            parent: None, left_child: None, right_child: None
        }))
    }

    fn insert(&mut self, new_ball: &Rc<RefCell<Ball>>, left: bool) {
        let new_ball = new_ball.borrow_mut();
        let dist = distance(&self.center, &new_ball.center);
        // if outside of the ball, make a new parent ball and put both inside
        if dist > self.radius {
            let mut new_parent = self.bounding_ball(&new_ball).borrow_mut();
            new_parent.parent = self.parent;
            new_parent.left_child = Some(Rc::new(RefCell::new(self.clone())));
            new_parent.right_child = Some(Rc::new(RefCell::new(new_ball.clone())));
            
            let new_parent = Rc::new(RefCell::new(new_parent.clone()));
            match self.parent {
                Some(ref old_parent) => {
                    let old_parent = old_parent.borrow_mut();
                    if left {
                        old_parent.left_child = Some(new_parent);
                    } else {
                        old_parent.right_child = Some(new_parent);
                    }
                },
                None => ()
            }
            self.parent = Some(new_parent);
            new_ball.parent = Some(new_parent);
        }
    }

    fn search_similar(&self, features: &Vec<f32>, limit: usize) {

    }

    // fn insert(&self, new_ball: Rc<RefCell<Ball>>) {
    //     let dist = distance(&self.center, &new_ball.center);
    //     // if outside of the ball, make a new parent ball and put both inside
    //     if dist > self.radius {
    //         let new_parent = self.bounding_ball(&new_ball);
    //         new_parent.parent = self.parent;
    //         new_parent.left_child = Some(&mut Box::new(*self));
    //         new_parent.right_child = Some(new_ball);
    //         self.parent = Some(&mut new_parent);
    //     }

    //     // if the ball contains other balls
    //     // if self.radius > 0. {

    //     //     let left_child = self.left_child.unwrap();
    //     //     let right_child = self.right_child.unwrap();

    //     //     // calculate the closest ball to insert the new ball into
    //     //     let left_dist = new_ball.distance(&left_child);
    //     //     let right_dist = new_ball.distance(&right_child);
    //     //     if left_dist < right_dist {
    //     //         left_child.insert(&new_ball);
    //     //     } else {
    //     //         right_child.insert(&new_ball);
    //     //     }
    //     // } else {
            
    //     // }
    // }
}


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
