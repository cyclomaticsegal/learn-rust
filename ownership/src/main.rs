fn main() {

    //let s1 = String::from("abc");
    //let s2 = s1; //gets moved from s1 to s2 and s1 becomes invalidated (dropped) by the compiler
    //s1 is no longer owns the value and this wont work
    //println!("value of s1:{}", s1);


    let s3 = String::from("abc");
    let s4 = s3.clone();

    println!("value of s3:{}", s3);
    println!("value of s4:{}", s4);

    let s5: String = String::from("abc");
    do_stuff(s5); //error (warning actually) moved

    //or we can make it mutable and move it in the between main and the function

    let mut s6: String = String::from("abc");
    s6 = do_stuff(s6); //gets moved to s and then returned back to s6
    println!("value of s6:{}", s6);
}

fn do_stuff(s: String) -> String {
    //do stuff
    s
}
