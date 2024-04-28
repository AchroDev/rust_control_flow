/*
*   Repitition with Loops
*/

/*
*   It’s often useful to execute a block of code more than once. For this task,
*   Rust provides several loops, which will run through the code inside the loop
*   body to the end and then start immediately back at the beginning.
*   Rust has three kinds of loops: loop, while, and for. Let’s try each one.
*/

fn loop_one() {
    loop {
        println!("again!");
        break;
    }
}

/*
*   When we run loop_one, we’ll see again! printed over and over continuously until
*   we stop the program manually. Most terminals support the keyboard shortcut ctrl-c to
*   interrupt a program that is stuck in a continual loop.
*/

// Fortunately, Rust also provides a way to break out of a loop using code.
// You can place the break keyword within the loop to tell the program when to stop executing the loop.

fn loop_two() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Entry point
fn main() {
    loop_one();
    loop_two();
}
