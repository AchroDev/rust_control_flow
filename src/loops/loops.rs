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

/*
   Returning Values from Loops
*/

/*
*   One of the uses of a loop is to retry an operation you know might fail, such as checking whether
*   a thread has completed its job. You might also need to pass the result of that operation out
*   of the loop to the rest of your code. To do this, you can add the value you want returned after
*   the break expression you use to stop the loop; that value will be returned out of the loop so you can use it.
*/

fn loop_two() {
    let mut counter = 0;

    let this = ();

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Note: the semicolon here is technically optional
        }
    };

    println!("The result is {result}");
}

/*
*   Before the result loop, we declare a variable named counter and initialize it to 0.
*   Then we declare a variable named result to hold the value returned from the loop.
*   On every iteration of the loop, we add 1 to the counter variable, and then check
*   whether the counter is equal to 10. When it is, we use the break keyword with the
*   value counter * 2. After the loop, we use a semicolon to end the statement that
*   assigns the value to result. Finally, we print the value in result, which in this case is 20.
*/

/*
    Loop Labels to Diambiguate Between Multiple Loops
*/

/*
*   If you have loops within loops, 'break' and 'continue' apply to the innermost loop
*   at that point. You can optionally specify a 'loop label' on a loop that you can then
*   use with 'break' or 'continue' to specify that those keywords apply to the nested
*   labeled loop instead of the innermost loop. Loop labels must begin with a signle quote.
*/

fn loop_three() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

/*
*   The outer loop has the label 'counting_up, and it will count up from 0 to 2.
*   The inner loop without a label counts down from 10 to 9. The first break that
*   doesn’t specify a label will exit the inner loop only. The break 'counting_up; statement
*   will exit the outer loop.
*/

// Entry point
fn main() {
    loop_one();
    loop_two();
    loop_three();
}
