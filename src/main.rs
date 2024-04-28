/*
*   An if expression allows you to branch your code depending on conditions.
*   You provide a condition and then state, “If this condition is met, run
*   this block of code. If the condition is not met, do not run this block of code.”
*/

//It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.

fn main() {
    let number = 7;

    if number < 5 {
        println! {"condition was true"};
    } else {
        println! {"condition was false"};
    }
}
