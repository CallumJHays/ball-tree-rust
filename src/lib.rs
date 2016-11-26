use std::cell::RefCell;
use std::rc::Rc;

// A hyperdimensional ball that may have other balls as children
#[derive(Debug, Clone)]
pub struct Ball {
    center: Vec<f32>,
    radius: f32,
    parent: Option<Rc<RefCell<Ball>>>,
    left_child: Option<Rc<RefCell<Ball>>>,
    right_child: Option<Rc<RefCell<Ball>>>,
}

fn main() {
    let b1 = Ball::new(vec![1., 3., 6., 3.]);
    println!("{:?}", b1);

    let b2 = Ball::new(vec![3., 5., 8., 5.]);
    println!("{:?}", b2);

    insert(&b1, &b2, false);
    println!("{:?}", b1);
}

impl Ball {
    fn new(features: Vec<f32>) -> Rc<RefCell<Ball>> {
        Rc::new(RefCell::new(Ball {
            center: features,
            radius: 0.,
            parent: None,
            left_child: None,
            right_child: None,
        }))
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
            parent: None,
            left_child: None,
            right_child: None,
        }
    }

    // fn search_similar(&self, features: &Vec<f32>, limit: usize) {

    // }
}


fn insert(this: &Rc<RefCell<Ball>>, new_ball: &Rc<RefCell<Ball>>, is_left: bool) {
    println!("Inserting {:?} into {:?}", new_ball, this);
    let dist = distance(&this.borrow().center, &new_ball.borrow().center);
    println!("Distance = {}", dist);
    // if outside of the ball, make a new parent ball and put both inside
    if dist > this.borrow().radius {
        println!("Distance > this.radius: {}. Wrapping in new ball..", this.borrow().radius);
        let bounding = Rc::new(RefCell::new(this.borrow().bounding_ball(&new_ball.borrow())));
        println!("bounding {:?}", bounding);
        println!("State of this {:?}", this);
        
        match this.borrow().parent {
            Some(ref old_parent) => {
                println!("old_parent exists! wrapping with middle man.");
                let mut bounding_mut = bounding.borrow_mut();
                let mut old_parent_mut = old_parent.borrow_mut();
                bounding_mut.parent = Some(old_parent.clone());
                if is_left {
                    old_parent_mut.left_child = Some(bounding.clone());
                } else {
                    old_parent_mut.right_child = Some(bounding.clone());
                };
                this.borrow_mut().parent = Some(bounding.clone());
                bounding_mut.right_child = Some(this.clone());
                bounding_mut.left_child = Some(new_ball.clone());
                new_ball.borrow_mut().parent = Some(bounding.clone());
            },

            None => {
                println!("State of this {:?}", this);
                println!("No parent found. Must be root ball.");
                let mut this_mut = this.borrow_mut();
                // Must be the first ball in the tree.
                println!("Borrowed this_mut");
                let this_copy = Rc::new(RefCell::new(Ball {
                    center: this_mut.center.clone(),
                    radius: this_mut.radius,
                    parent: this_mut.parent.clone(),
                    left_child: this_mut.left_child.clone(),
                    right_child: this_mut.right_child.clone()
                }));
                println!("Cloned self: {:?} {:?}", this, this_copy);
                this_mut.left_child = Some(this_copy);
                println!("Set new left_child");
                this_mut.right_child = Some(new_ball.clone());
                println!("Set new right_child");
                this_mut.center = bounding.borrow().center.clone();
                println!("Set new center");
                this_mut.radius = bounding.borrow().radius;
                println!("Set new radius");
            }
        }
    } else { // inject it into the closest child
        // let left_child_center = this.borrow().left_child.unwrap().borrow().center;
        // let right_child_center = this.borrow().right_child.unwrap().borrow().center;
        // let left_dist = distance(
        //     &left_child_center,
        //     &new_ball.borrow().center);
        // let right_dist = distance(
        //     &right_child_center,
        //     &new_ball.borrow().center);
        // if left_dist < right_dist {
        //     insert(&this, &new_ball, true);
        // } else {
        //     insert(&this, &new_ball, false);
        // }
    }
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
