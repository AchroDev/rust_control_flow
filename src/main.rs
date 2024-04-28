/*
*   An if expression allows you to branch your code depending on conditions.
*   You provide a condition and then state, “If this condition is met, run
*   this block of code. If the condition is not met, do not run this block of code.”
*/

//It’s also worth noting that the condition in an if expression must be a bool. If the condition isn’t a bool, we’ll get an error.

fn example2() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

/*
*   Handling Multiple Conditions with "else if"
*/

fn example3() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Entry point
fn main() {
    let number = 7;

    if number < 5 {
        println! {"condition was true"};
    } else {
        println! {"condition was false"};
    }

    example2();
    example3();
}
