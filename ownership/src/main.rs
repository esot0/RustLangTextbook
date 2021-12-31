/*
Rather than having the user ensure memory safety as in C or C++, or having
a garbage collector like Java, Rust uses an ownership system where data must follow a set of rules
that the compiler checks at compile time


   > Each value in Rust has a variable that’s called its owner.
   >  There can only be one owner at a time.
   >  When the owner goes out of scope, the value will be dropped.

   You can have random little brackets to denote an inner scope anywhere
   {


   }

   rust automatically calls a "drop" function which seems to be analagous to free in cpp/c

   For objects

   let s1 = String::from("hello");
   let s2 = s1;

   pointer of s2 = pointer of s1

   when s1 goes out of scope, rust doesn't free anything as it considers it invalid
   So drop is only called when s2 goes out of scope
   You also can't use s1 past this point
*/

fn main() {

    let s = "hello";
    let mut s = String::from("hello");
    s.push_str(",world!");

    //The below is a shallow copy. It just copies the pointer, its length, and the size the obj takes up in memory. a is invalidated.
    let a = String::from("hello");
    let b = a;

    println!("{}, I'm a shallow copy" , b);
    //The below is a deep copy. The values of c are copied as well as the things copied in a shallow copy. c is not invalidated.

    let c = String::from("hey there");
    let d = c.clone();
    println!("{} I'm a copy! {} I'm the original!", d,c);
    //Integers are fine to use after a shallow copy

    //Passing a variable into a function makes a shallow copy


    /*
    Yet, a lot of times, you'll want to use a variable in an outer scope
    after you pass it into a function. You can't just set things equal to each other so..
    How do we do it in rust?

    WELL, how do you do it in C++? That's right baby, references are back and ready to attack!!
    */

    let ref_string = String::from("hello");
    let len = calculate_length(&ref_string);

    println!("The length of '{}' is {}", ref_string, len);
    
    let mut changing_ref_str = String::from("String cheese");
    defile_string(&mut changing_ref_str);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

/*
NOTE, we're only "borrowing" a value like this. 
If we try to change it in any way, we'll get an error
But, there is a way to change its unchangingness
Mut!

However, you can't have more than one mutable ref to a particular piece of data at a time

[

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);


]

The above would net you an error
*/

fn defile_string(s: &mut String) {
    s.push_str(" has been [ALTERED]");
}