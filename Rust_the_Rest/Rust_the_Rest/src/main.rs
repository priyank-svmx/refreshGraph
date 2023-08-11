fn main( ){
    println!("Hello, world! sweet heart!I");
    println!("My name is {} and i am {} old", "micke", 23);
let apples = 10;
let oranges = 4;

println!("{} these many apples i have along with {} oranges", apples, oranges);

let testingNumericSum = apples + oranges;


// this is tuple not the comma operator in rust shit!
let testInterpolationUsingCommaOperator = ("{},{}", apples, oranges);


//println!("{}",testInterpolationUsingCommaOperator);
//
//
let apples = 24;
let apples = 45;
let apples = 36;

let name: &str = "help";
let number: i32 = 124;

println!("number {} abd the string {}",name,number);

let x: () = ();
// since default formatter cannot print the unit type ():
//println!("trying to print a unit value {}", x);

 let x:  &str = {

     println!("here i am printing values inside a block, and blocks are jsut like ocaml and scala");
     println!("let's return the value as string and see what error i get");
     "helper"    
 };



}
