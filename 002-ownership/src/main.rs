/* 
// 
fn main() {

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}
*/

/*
//-----------------------------------------
// clone vs =
fn main() {
    let s1 = String::from("hello");
    //let s2 = s1; //s1 is not usable anymore
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);
}
*/

/*
//-----------------------------------------
// ownership
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    //println!("{}", s); // would crash

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("{}", x);                                    

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
*/

/*
//-----------------------------------------
// ownership and scope
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("{}",s3)
} 

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    println!("from takes and gives back : {}",a_string.clone());
    a_string  // a_string is returned and moves out to the calling function
}
*/

/*
// reference and borrowing
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/

//muttable ref
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}",s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
