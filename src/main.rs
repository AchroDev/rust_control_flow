/*
*   An if expression allows you to branch your code depending on conditions.
*   You provide a condition and then state, “If this condition is met, run
*   this block of code. If the condition is not met, do not run this block of code.”
*/

fn main() {
    let number = 3;

    if number < 5 {
        println! {"condition was true"};
    } else {
        println! {"condition was false"};
    }
}
