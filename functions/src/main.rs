fn main() {
    println!("Hello, world!");

    // println!("{}", sum(3,3));
    // println!("{}", five());

    //both outcomes of a ternary operator must be of the same type
    // let condition = false;
    // let number = if condition {7} else {8};

    // println!("The condition evals to {}", number);

}

//rust does not require explicit return statements. usually, the last line in a function is the returned value
//parameters MUST have their type declared in function signatures

fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

//this function is perfectly fine
fn five() -> i32 {
    5
}

fn unconditional_loop() {
    //loops forever until forcibly stopped via a break of ctrl + c
    //continue will continue the loop
    loop {
        println!("looping")
    }
}

fn loop_label() {
    //the single quotation mark indicates a loop label, which allows you to break the outer loop from an inner loop
    //usually, excessive breaks and continues in other languages are a sign that your implementation isn't the best. Is there an
    //advantage to them in rust?
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);
}

fn loop_return() {
    //you can return values from loops
    let mut count = 0;

    let result = loop {
        count+=1;

        if count == 10 {
            break count * 2; // stops loop and returns final count * 2
        }
    };

    println!("The result is {}", result);
}

fn while_loop(){
    let mut index = 0;
    let a = [1,2,3,4,5,6];

    while index < a.len() {
        println!("The value at index {} is {}", index, a[index]);
        index+=1;
    }
}

fn for_loop(){
    let a = [1,2,3,4,5,6];

    for element in a {
        println!("the value is: {}", element);
    }
}

//you can use a range to loop thru the numbers from the first to the last (non inclusive)
//rev loops in reverse order. seems to only work on iterators
fn liftoff() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF :) !!!")
}