fn multiplication(a: i32, b: i32) -> i32 {
	a * b
}

fn multiplication_print_result(a: i32, b: i32){
	println!("{}", multiplication(a, b));
}

mod some_module;

fn main() {
    let x = 2;

    let y = 3;

    some_module::say_hello();

    //println!("{:?}", multiplication(x, y));
}
