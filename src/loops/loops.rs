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

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // NOTE: the semicolon here is technically optional
        }
        /*
         *   Code after a break or return is never executed, so the Rust compiler treats
         *   a break expression and a return expression as having the value unit, or ().
         */
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

/*
    Conditional Loops with While
*/

/*
*   A program will often need to evaluate a condition within a loop. While the condition
*   is 'true', the loop runs. When the condition ceases to be 'true', the program calls 'break',
*   stopping the loop. It's possible to implement the behaviour like this using a combination of
*   'loop', 'if', 'else, and 'break'; you could try that now in a program, if you'd like.
*   However, this pattern is so common that Rust has a built-in language construct for it,
*   called a 'while' loop. In loop_four, we use 'while' to loop the program three times,
*   counting down each time, and then, after the loop, print a message and exit.
*/

fn loop_four() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

/*
*   This construct eliminates a lot of nesting that would be necessary if you used
*   'loop', 'if', 'else', and 'break', and it's clearer. While a condition evaluates
*   to 'true', the code runs; otherwise, it exits the loop.
*/

/*
    Looping Through a Collection with for
*/

/*
*   You can choose to use the 'while' construct to loop over the elements of a collection,
*   such as an array. For example, the loop in loop_five prints each element in the array a.
*/

fn loop_five() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

/*
*   Here, the code counts up through the elements in the array. It starts at index 0,
*   and then loops until it reaches the final index in the array(that is, when 'index < 5)
*   is no longer 'true'). Running this code will print every element in the array.
*/

/*
*   All five array values appear in the terminal, as expected. Even though 'index' will
*   reach a value of '5' at some point, the loop stops executing before trying to fetch
*   a sixth value from the array.
*
*   However, this approach is error prone; we could cause the program to panic if the
*   index value or test condition is incorrect. For example, if you changed the definition
*   of the 'a' array to have four elements but forgot to update the condition to
*   'while index < 4', the code would panic. It's also slow, because the compiler adds runtime
*   code to perform the conditional check of whether the index is within bounds of the array
*   on every iteration through the loop.
*
*   As a more concise alternative, you can use a 'for' loop and execute some code for each
*   item in a colletion. A 'for' loop looks like the code in loop_six."
*/

fn loop_six() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

/*
*   When we run this code, we'll see the same output as in loop_five. More importantly,
*   we've now increased the safety of the code and elminated the chance of bugs that might
*   result from going beyond the end of the array or not going far enough and missing some items.

*   Using the 'for' loop, you wouldn't need to remember to change any other code if you changed
*   the number of values in the array, as you would with the method used in loop_five.

*   The safety and conciseness of 'for' loops make them the most commonly used loop construct
*   in Rust. Even in situations in which you want to run some code a certain number of times, as in
*   the countdown example that used a 'while' loop in loop_four, most Rustaceans would use a 'for' loop.
*   The way to do that would be to use a 'Range', provided by the standard library, which
*   generates all number in sequence starting from one number and ending before another number.

*   Here's what the countdown would look like using a 'for' loop and another method we've not
*   yet talked about, 'rev', to reverse the range:
*/

fn loop_seven() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// Entry point
fn main() {
    loop_one();
    loop_two();
    loop_three();
    loop_four();
    loop_five();
    loop_six();
    loop_seven();
}
