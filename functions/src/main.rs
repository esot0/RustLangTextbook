fn main() {
    // println!("Hello, world!");
    // println!("{}", fibonacci(6));

    // println!("{}", sum(3,3));
    // println!("{}", five());

    //both outcomes of a ternary operator must be of the same type
    // let condition = false;
    // let number = if condition {7} else {8};

    // println!("The condition evals to {}", number);

    // twelve_days_of_christmas();

    println!("{}", fahrenheit_to_celsius(98.6))
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

fn fibonacci(n: i32) -> i32 {
    let mut counter = 1;
    let mut sum = 0;
    let mut prevsum1 = 1;
    let mut prevsum2 = 0;
    
    if (n<=2){
        return 1;
    }

    while counter < n {
        sum = prevsum1 + prevsum2;
        prevsum2 = prevsum1;
        prevsum1 = sum;
        counter+=1;
    }
    

    return sum;
}

fn twelve_days_of_christmas(){
    let lyrics = ["A partridge in a pear tree", "Two turtle doves, and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for num in (1..13){
        print!("On the {}", num);
        day_ending(num);

        let counter  :usize = num.try_into().unwrap();
        for number in (1..counter+1).rev(){
            println!("{}", lyrics[number-1]);
        }
    }
}

fn day_ending(num: i32){
    if(num==1){
        print!("st day of christmas my true love gave to me\n");
    }
    else if (num==2){
        print!("nd day of christmas my true love gave to me\n");
    }
    else if (num==3){
        print!("rd day of christmas my true love gave to me\n");
    }
    else {
        print!("th day of christmas my true love gave to me\n");
    }
}

fn fahrenheit_to_celsius(fahrenheit:f32) -> f32{
    return (fahrenheit-32.0)/1.8;
}