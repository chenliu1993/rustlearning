fn main() {
   println!("{}", generate_fib(5.to_string()));
}
fn generate_fib(n: String) -> String {
    let mut middle: u32 = n.trim().parse()
        .expect("Faile to convert the input");
    
    let mut ans: String = String::new();
    let mut test: String = String::new();
    while middle > 1 {
        test = middle.to_string() + "*";
        ans = ans + &test;
        middle -= 1;
    }
    test = middle.to_string();
    
    return  ans + &test
}
// fn another_fn(x: i32, y: i32) {
//     println!("The input is {} and {}", x, y);
//     let tup: (f32, i32) = (3.0, 5);
//     println!("{}", tup.0);
// }

// fn add_one() -> i32 {
//     let y = {
//         let x = 5;
//         x + 1
//     };

//     return y;
// }