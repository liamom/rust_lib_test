//use test_lib;
extern crate test_lib;
extern crate cpp_lib;
use test_lib::math_test::test_lib::test_add;

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("number = {}", x);

    let r = test_add(1,2);
    println!("number = {}", r);

    let cpp_number = cpp_lib::pub_lib_cpp_add(2, 5);
    println!("cpp number = {}", cpp_number);
}
