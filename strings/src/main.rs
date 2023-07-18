fn main() {
    //six types of strings

    //str string slice
    //&str (borrowed string slice)

    //"this literal string is always a borrowed string slice"

    //borrowed string slice is often referred to as a string

    //String 

    //a borrowed string slice cannot be modified
    //a String can be modified

    
    let msg: String = "this is borrowed".to_string();
    
    println!("the string {}", msg);
    let msg = "was modified".to_string();

    println!("the string {}", msg);

    let msg2 = String::from("hello simon");

    println!("{}", msg2);

    //&str is a pointer to bytes and a length (ie fixed length)
    //String has ptr to bytes, length and capacity (ie string can grow)

    //both are valid UTF8

    let last = "messages".chars().nth(0);
    match last {
        Some(c) => println!("{}", c),
        None => println!("No character found at zero index"),
    }
    

}
