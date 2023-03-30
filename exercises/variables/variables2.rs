// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.


fn main() {
    let x = 3;

    let x = x + 7;

    {
        let x = x * 2;
        println!("x is: {}", x);
    }

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
