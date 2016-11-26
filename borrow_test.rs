#[derive(Debug)]
struct Test {
    x: i32
}

fn main() {
    let test_struct = Test { x: 10 };
    let x = borrow_test(test_struct);
    println!("{:?}", x);
}

fn borrow_test(test: Test) -> Test {
    let foo = |test: &Test| test.x + 5;
    let y = foo(&test);
    println!("{}", y);
    if y > 12 {
        test
    } else {
        Test { x: 12 }
    }
}
