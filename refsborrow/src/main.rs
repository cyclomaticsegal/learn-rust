fn main() {
    let s1 = String::from("abc");
    //s1 retains ownership
    do_stuff(&s1);

    other_main_for_mut_ref();
}

//borrows a reference to the value and the ref is moved into the function and when leaves scope of function the ref is dropped
fn do_stuff(s: &String){
    //do stuff
    println!("{} is the value of s1", s);
}//borrowing ends here


fn other_main_for_mut_ref(){
    let mut s1 = String::from("abc");
    //mut_ref_example(&s1); // error
    mut_ref_example(&mut s1); // error
}

//ERROR example
// fn mut_ref_example(s: &String){
//     s.insert_str(0, "Hi, "); //error
// }

//mutable reference version works
//if you make a mutable reference to a mutable value then you use the ref to change the value as well!
fn mut_ref_example(s: &mut String){
    s.insert_str(0, "Hi, "); //error
    //. operator will force dereferencing
    //(*s).insert_str(0, "Hi, "); //also works

    //manually dereference if you want to read from or write to the value
    //this replaces the entire value
    *s = String::from("Replacement");
    println!("{} is the value of s", s);
}



/*
/// &x immutable ref to x's value
/// &mut x mutable ref x's value
/// 
/// 
/// &String is an immutable type reference
// &mut String is a mutable type reference
*/
