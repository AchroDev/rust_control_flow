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

/*
*   When this program executes, it checks each if expression in turn and executes
*   the first body for which the condition evaluates to true. Note that even though
*   6 is divisible by 2, we don’t see the output number is divisible by 2, nor do we
*   see the number is not divisible by 4, 3, or 2 text from the else block.
*   That’s because Rust only executes the block for the first true condition,
*   and once it finds one, it doesn’t even check the rest.
*/

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
