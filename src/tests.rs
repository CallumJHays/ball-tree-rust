use ball_tree::BallTree;
use ball_tree::BallTree::*;
use vector_math::*;

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
    let test = add_scal(&test_vec(), &42.);
    assert_eq!(test, vec![43., 44., 45., 46.]);
}

#[test]
fn subtract_vec_test() {
    let test = subtract_vec(&test_vec(), &test_vec());
    assert_eq!(test, vec![0.; 4]);

    let test = subtract_vec(&test_vec(), &vec![11., 12., 13., 14.]);
    assert_eq!(test, vec![-10.; 4]);
}

#[test]
fn subtract_scal_test() {
    let test = subtract_scal(&test_vec(), &100.);
    assert_eq!(test, vec![-99., -98., -97., -96.]);
}

#[test]
fn multiply_scal_test() {
    let test = multiply_scal(&test_vec(), &20.);
    assert_eq!(test, vec![20., 40., 60., 80.]);
}

#[test]
fn divide_scal_test() {
    let test = divide_scal(&test_vec(), &10.);
    assert_eq!(test, vec![0.1, 0.2, 0.3, 0.4]);
}

#[test]
fn magnitude_test() {
    let test = magnitude(&vec![2., 2., 2., 2.]);
    assert_eq!(test, 4.);
}

#[test]
fn distance_test() {
    let test = distance(&vec![4., 3.], &vec![-4., -3.]);
    assert_eq!(test, 10.);
}

#[test]
fn midpoint_test() {
    let test = midpoint(&vec![50., 10., 10.], &vec![-50., 10., 10.]);
    assert_eq!(test, vec![0., 10., 10.]);
}

#[test]
fn new_ball_tree_test() {
    let nil_tree = BallTree::new();
    assert_eq!(nil_tree, Nil);
}

// this is a mirror of the private BallTree impl function.
// should be kept up to date at all times
fn bounding_ball(me: BallTree, other: BallTree) -> BallTree {
    let get_center_and_radius = |tree: &BallTree| -> (Vec<f32>, f32) {
        match *tree {
            Point(ref center) => (center.clone(), 0.),
            Ball(ref center, ref rad, _, _) => (center.clone(), *rad),
            Nil => panic!("The supplied tree is Nil!")
        }
    };

    let (self_center, self_rad) = get_center_and_radius(&me);
    let (other_center, other_rad) = get_center_and_radius(&other);

    let span = subtract_vec(&self_center, &other_center);
    let mag = magnitude(&span);
    let unit_vec = divide_scal(&span, &mag);
    let p1 = add_vec(&self_center, &multiply_scal(&unit_vec, &self_rad));
    let p2 = subtract_vec(&other_center, &multiply_scal(&unit_vec, &other_rad));
    Ball(midpoint(&p1, &p2), distance(&p1, &p2) / 2., Box::new(me), Box::new(other))
}

#[test]
fn bounding_ball_test() {
    let p1 = || Point(vec![2., 2., 2., 2.]);
    let p2 = || Point(vec![-2., -2., -2., -2.]);

    assert_eq!(
        bounding_ball(p1(), p2()),
        Ball(vec![0., 0., 0., 0.], 4.,
            Box::new(p1()),
            Box::new(p2())
        )
    );
}

#[test]
fn ball_tree_push_test() {
    let nil_tree = BallTree::new();

    // pushing to an empty tree will yield the point
    let vec1 = || vec![1., 2., 3., 4.]; // use vector factory for brevity
    let tree1 = nil_tree.push(&vec1());
    assert_eq!(tree1, Point(vec1()));

    // pushing to a tree with a point at root will yield a ball with two points
    let vec2 = || vec![-5., 6., -7., 8.];
    let ball2 = || bounding_ball(Point(vec2()), Point(vec1()));
    let center2 = || {
        match ball2() {
            Ball(center, _, _, _) => center,
            _ => panic!("What the...")
        }
    };
    let radius2 = || {
        match ball2() {
            Ball(_, radius, _, _) => radius,
            _ => panic!("What the...")
        }
    };
    let tree2 = tree1.push(&vec2());
    assert_eq!(tree2,
        Ball(
            center2(),
            radius2(),
            Box::new(Point(vec1())),
            Box::new(Point(vec2()))
        )
    );

    // pushing to a tree with a ball at root will yield a nested ball tree structure
    let vec3 = || vec![9., -10., 11., -12.];
    let ball3 = || bounding_ball(Point(vec3()), ball2());
    let center3 = || {
        match ball3() {
            Ball(center, _, _, _) => center,
            _ => panic!("What the...")
        }
    };
    let radius3 = || {
        match ball3() {
            Ball(_, radius, _, _) => radius,
            _ => panic!("What the...")
        }
    };
    let tree3 = tree2.push(&vec3());
    assert_eq!(tree3,
        Ball(
            center3(),
            radius3(),
            Box::new(Ball(
                center2(),
                radius2(),
                Box::new(Point(vec1())),
                Box::new(Point(vec2()))
            )),
            Box::new(Point(vec3()))
        )
    );
}

fn simple_sample_tree() -> BallTree {
    let features = [
        vec![15., -7.],
        vec![-20., -20.],
        vec![9., 10.],
        vec![6., 6.],
        vec![-2., 10.],
        vec![18., 5.],
        vec![1., 1.],
    ];

    let mut bt = BallTree::new();
    for feature in features.iter() {
        bt = bt.push(&feature);
    }

    bt
}

#[test]
fn ball_tree_nn_search_test() {
    let bt = simple_sample_tree();
    let search_feature = vec![10., 10.];
    let expected_res = vec![
        vec![9., 10.],
        vec![6., 6.],
        vec![18., 5.],
        vec![-2., 10.],
        vec![1., 1.],
    ];
    
    assert_eq!(bt.nn_search(&search_feature, &5), expected_res);
}
