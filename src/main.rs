fn main() {
    /* -- if -- */
    let number = 7;
    if number < 5 {
        println!("Less than 5");
    } else if number == 5 {
        println!("Exactly 5");
    } else {
        println!("Greater than 5");
    }

    /* -- match -- */
    let grade = 'B';

    match grade {
        'A' => println!("Excellent"),
        'B' => println!("Good job"),
        'C' => println!("Average"),
        _ => println!("Needs improvement"), // match must be exhaustive
    }

    /* loop */

    let mut infinite_counter = 0;

    loop {
        // infitnite loop
        println!("loop_counter: {}", infinite_counter);
        infinite_counter += 1;
        if infinite_counter == 3 {
            break; // must have a break case when running infinite loops
        }
    }

    let mut while_counter = 3;
    while while_counter > 0 {
        println!("while_counter: {}", while_counter);
        while_counter -= 1;
    }
    println!("Lift Off!");

    /* -- for -- */
    for i in 1..=5 {
        // =5 means inclusive, otherwise it won't include final number
        println!("for loop number is: {}", i);
    }
}

/*
Chapter 3 Quiz

    1. What keyword starts an infinite loop in Rust?

    2. What’s the difference between if and match?

    3. What loop would you use to iterate over a range?

    4. What does the _ symbol mean in a match block?


Answers:
1. the keyword is 'loop'

2. if checks for boolean conditions; match compares pattern and values

3. to iterate over a range, for loop is best as to not go out of bounds

4. it is a catch-all/default case for match pattern




*/
