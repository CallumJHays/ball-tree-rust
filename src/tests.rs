use ball_tree::BallTree;
use ball_tree::BallTree::*;
use vector_math::*;

//
//--------------- VECTOR MATH TESTS ---------------
//

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

//
//--------------------- BALL TREE TESTS ------------------------
//

#[test]
fn bounding_ball_test() {
    let p1 = || Point(vec![2., 2., 2., 2.]);
    let p2 = || Point(vec![-2., -2., -2., -2.]);

    assert_eq!(
        p1()._bounding_ball(p2()),
        Ball(vec![0., 0., 0., 0.], 4.,
            Box::new(p1()),
            Box::new(p2())
        )
    );

    let ball1 = || p1()._bounding_ball(p2());
    let p3 = || Point(vec![12., 0., 0., 0.]);

    assert_eq!(
        p3()._bounding_ball(ball1()),
        Ball(vec![4., 0., 0., 0.], 8.,
            Box::new(p3()),
            Box::new(Ball(vec![0., 0., 0., 0.], 4.,
                Box::new(p1()),
                Box::new(p2())
            ))
        )
    )
}

#[test]
fn ball_tree_push_test() {
    let nil_tree: BallTree<Vec<f32>> = BallTree::new();

    // pushing to an empty tree will yield the point
    let vec1 = || vec![1., 2., 3., 4.]; // use vector factory for brevity
    let tree1 = nil_tree.push(&vec1());
    assert_eq!(tree1, Point(vec1()));

    // pushing to a tree with a point at root will yield a ball with two points
    let vec2 = || vec![-5., 6., -7., 8.];
    let ball2 = || Point(vec2())._bounding_ball(Point(vec1()));
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
    let vec3 = || vec![0., 2., 3., 4.];
    let ball3 = || Point(vec1())._bounding_ball(Point(vec3()));
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
            center2(),
            radius2(),
            Box::new(Ball(
                center3(),
                radius3(),
                Box::new(Point(vec1())),
                Box::new(Point(vec3()))
            )),
            Box::new(Point(vec2()))
        )
    );

    // push a point that fits outside of left but closer to right.
    let vec4 = || vec![-5., 6., -7., 7.];
    let ball4 = || Point(vec2())._bounding_ball(Point(vec4()));
    let center4 = || {
        match ball4() {
            Ball(center, _, _, _) => center,
            _ => panic!("What the...")
        }
    };
    let radius4 = || {
        match ball4() {
            Ball(_, radius, _, _) => radius,
            _ => panic!("What the...")
        }
    };
    let tree4 = tree3.push(&vec4());
    assert_eq!(tree4,
        Ball(
            center2(),
            radius2(),
            Box::new(Ball(
                center3(),
                radius3(),
                Box::new(Point(vec1())),
                Box::new(Point(vec3()))
            )),
            Box::new(Ball(
                center4(),
                radius4(),
                Box::new(Point(vec2())),
                Box::new(Point(vec4()))
            ))
        )
    )
}

fn simple_sample_features() -> [Vec<f32>; 7] {
    [
        vec![15., -7.],
        vec![-20., -20.],
        vec![9., 10.],
        vec![6., 6.],
        vec![-2., 10.],
        vec![18., 5.],
        vec![1., 1.],
    ]
}

fn simple_sample_tree() -> BallTree<Vec<f32>> {
    let features = simple_sample_features();

    let mut bt: BallTree<Vec<f32>> = BallTree::new();
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

#[test]
fn ball_tree_flatten_test() {
    let bt = simple_sample_tree();
    let flattened = bt.flatten();

    let expected_features = simple_sample_features();

    assert_eq!(flattened.len(), expected_features.len());

    for feature in expected_features.iter() {
        assert!(flattened.contains(&feature));
    }
}

#[test]
fn ball_tree_load_test() {
    let features = simple_sample_features();
    let bt = BallTree::load(&features.to_vec());

    let flattened = bt.flatten();

    assert_eq!(flattened.len(), features.len());
    for feature in features.iter() {
        assert!(flattened.contains(&feature));
    }
}

//
//------------------ BENCHMARK TESTS --------------------
//

extern crate rand;
extern crate test;

use self::rand::Rng;

fn gen_random_vector(dimensions: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dimensions)
    .map(|_| rng.gen::<f32>())
    .collect()
}

fn gen_random_vectors(num: usize, dimensions: usize) -> Vec<Vec<f32>> {
    (0..num).map(|_| gen_random_vector(dimensions)).collect()
}

fn random_benchmark_tree(size: usize, length: usize) -> BallTree<Vec<f32>> {
    BallTree::load(&gen_random_vectors(size, length))
}

fn pow2(power: u32) -> usize { (2 as usize).pow(power) }

// ##################### RANDOM TREE BENCHMARK #####################
/*
Every test iteration needs to generate a new random tree. So every time
the something with a new random tree is generated, subtract the average
tree generation performance to find the average actual performance.

Free memory required to run an AxB benchmark is (roughly):
        mem. req = 8 * 2^A * 2^B / 1024^2 MB

*/
//--------------------- 256 length vectors ---------------------
// #[bench]
// fn clone_tree_10x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_14x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_18x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_20x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(20);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }

// //--------------------- 512 length vectors ---------------------
// #[bench]
// fn clone_tree_10x9_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_14x9_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_18x9_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
#[bench]
fn clone_tree_20x9_bench(b: &mut test::Bencher) {
    let SIZE = pow2(20);
    let LENGTH = pow2(9);
    let bt = random_benchmark_tree(SIZE, LENGTH);
    b.iter(|| bt.clone())
}

// //--------------------- 1024 length vectors ---------------------
// #[bench]
// fn clone_tree_10x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_14x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_18x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }
// #[bench]
// fn clone_tree_20x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(20);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone())
// }

// //################## PUSH BENCHMARKS ##########################
// //----------------- 256 length vector benchmarks --------------

// #[bench]
// fn ball_tree_push_10x8_bench(b: &mut test::Bencher) {
//     let LENGTH = pow2(10);
//     let SIZE = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_14x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_18x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_20x8_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(20);
//     let LENGTH = pow2(8);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// //----------------- 512 length vector benchmarks --------------

// #[bench]
// fn ball_tree_push_10x9_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_14x9_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_18x9_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(9);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

#[bench]
fn ball_tree_push_20x9_bench(b: &mut test::Bencher) {
    let SIZE = pow2(20);
    let LENGTH = pow2(9);
    let bt = random_benchmark_tree(SIZE, LENGTH);
    b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
}

// //----------------- 1024 length vector benchmarks --------------

// #[bench]
// fn ball_tree_push_10x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(10);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_14x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(10);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_18x10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(10);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// #[bench]
// fn ball_tree_push_20x10_bench(b: &mut test::Bencher) {
//     let LENGTH = pow2(20);
//     let SIZE = pow2(10);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().push(&gen_random_vector(LENGTH)))
// }

// ################# SEARCH BENCHMARKS ######################
// ############## USING 512 LENGTH VECTORS ##################

// ---------------------- TOP 1 --------------------------

// #[bench]
// fn ball_tree_search_0_from_10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(9);
//     let SEARCH = pow2(0);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// #[bench]
// fn ball_tree_search_0_from_14_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(9);
//     let SEARCH = pow2(0);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// #[bench]
// fn ball_tree_search_0_from_18_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(9);
//     let SEARCH = pow2(0);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// // ---------------------- TOP 4 --------------------------

// #[bench]
// fn ball_tree_search_2_from_10_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(10);
//     let LENGTH = pow2(9);
//     let SEARCH = pow2(2);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// #[bench]
// fn ball_tree_search_2_from_14_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(14);
//     let LENGTH = pow2(9);
//     let SEARCH = pow2(2);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// #[bench]
// fn ball_tree_search_2_from_18_bench(b: &mut test::Bencher) {
//     let SIZE = pow2(18);
//     let LENGTH = pow2(9);
//     let SEARCH = pow2(2);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// // ---------------------- TOP 16 --------------------------

// #[bench]
// fn ball_tree_search_4_from_10_bench(b: &mut test::Bencher) {
//     let SEARCH = pow2(4);
//     let LENGTH = pow2(9);
//     let SIZE = pow2(10);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// #[bench]
// fn ball_tree_search_4_from_14_bench(b: &mut test::Bencher) {
//     let SEARCH = pow2(4);
//     let LENGTH = pow2(9);
//     let SIZE = pow2(14);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }

// #[bench]
// fn ball_tree_search_4_from_18_bench(b: &mut test::Bencher) {
//     let SEARCH = pow2(4);
//     let LENGTH = pow2(9);
//     let SIZE = pow2(18);
//     let bt = random_benchmark_tree(SIZE, LENGTH);
//     b.iter(|| bt.clone().nn_search(&gen_random_vector(LENGTH), &SEARCH))
// }
