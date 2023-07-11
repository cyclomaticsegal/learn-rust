use hello::greet;
use rand::{thread_rng, Rng};
fn main() {
    //absolute path to the function
    //lib name wich is same as project name (hello)
    //the scope operator ::
    //then name of function
    //requires greet to be public too

    //also absolute path could be painful if path is very long
    //in which case use the use statement
    //hello::greet();

    greet();
    let x = thread_rng().gen_range(0..100);
    println!("x is {}", x.to_string());
}
