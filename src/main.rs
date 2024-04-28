/*
*   An if expression allows you to branch your code depending on conditions.
*   You provide a condition and then state, “If this condition is met, run
*   this block of code. If the condition is not met, do not run this block of code.”
*/

//It’s also worth noting that the condition in an if expression must be a bool. If the condition isn’t a bool, we’ll get an error.

fn example_two() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

/*
*   Handling Multiple Conditions with "else if"
*/

fn example_three() {
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
*   When the above program executes, it checks each if expression in turn and executes
*   the first body for which the condition evaluates to true. Note that even though
*   6 is divisible by 2, we don’t see the output number is divisible by 2, nor do we
*   see the number is not divisible by 4, 3, or 2 text from the else block.
*   That’s because Rust only executes the block for the first true condition,
*   and once it finds one, it doesn’t even check the rest.
*/

/*
*   Using if in a let statement
*/

// Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
fn example_four() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

/*
*   Remember that blocks of code evaluate to the last expression in them,
*   and numbers by themselves are also expressions. In this case, the value
*   of the whole if expression depends on which block of code executes.
*   This means the values that have the potential to be results from each arm
*   of the if must be the same type
*/

// Entry point
fn main() {
    let number = 7;

    if number < 5 {
        println! {"condition was true"};
    } else {
        println! {"condition was false"};
    }

    example_two();
    example_three();
    example_four();
}
