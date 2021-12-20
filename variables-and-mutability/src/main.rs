//You will run into errors if mut keyword does not preceed variable

fn func(){
    println!("Other function")
}

fn main() {
    //constants are a little different from normal rust vars. You may not use mut, and the datatype must be annotated

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /*By using the let x expression twice, we are shadowing x
    Shadowing is different from mutability. If we try to reassign w/o using let we'll get a compile-time err
    In using let, we can change the var but have it be immutable *after* those chanes
    */

    let x = 5;

    let x = x + 1;

    {
        //The brackets define an inner scope inside the function
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

   // We can also change the type of the variable but reuse the same name since we're essentially making a new variable

   let spaces = " ";
   let spaces = spaces.len();

   //If we used mut here, we'd have an error

   //Rust has both explicitly defined data types and inferred data types
   //You may need to explicity define a type when many are possible, i.e, in a parse, do you want a 32 bit int? 64 bit? etc ect

   let guess: u32 = "42".parse().expect("Not a number!");

    //creates an array with the num 3 repeated 5 times
   let arr = [3;5];
   let arr2 = [1,2,3,4,5];
   let arr3: [i32; 5] = [1,2,3,4,5];

   for nums in arr.iter(){
       println!("{}",nums);
   }

   //statements like let x = y = 6 do not work in rust, as they would in C or Ruby
   //Unlike in those languages, those statements don't return anything in rust.

   func();

   let a = {
       let b = 3;
       b + 1;
   };
    //You can define variables according to expressions such as the above (which evaluates to 4)
   //arch datatype = size of datatype is derived from machine architecture
   //an arch bit will be 32 bits on an x86 machine and 64 bits on an x64 bits

   let index: usize = 2;
}


