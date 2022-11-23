// Functions - Used to store blocks of code for re-use 

pub fn _run() {
    greeting("Good Morning", "António Gabriel");

    // Sum operation
    let sum_result = add(12, 22);
    println!("Result is {sum_result}");

    // Clojure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(12, 33));
}

fn greeting(greet: &str, name: &str) {
    println!("{greet} {name}, nice to meet you!");
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}