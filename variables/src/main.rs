fn main() {
    //infered to int 
    //let bunnies = 2;
    //let bunnies: i32 = 2;

    //destructure with let statement
    // let (bunnies, carrots) = (8, 50);
    // let b: String = bunnies.to_string();
    // println!("{}", b);
    // println!("{}", carrots.to_string());

    //mutability (immutable by default)
    //safety concurrency and speed
    //optimization by compiler
    // let mut bunnies = 32;
    // println!("{}", bunnies.to_string()); // if not include then the rustc would complain that it was overwrittem before being used
    // bunnies = 2;
    // println!("{}", bunnies.to_string());


    //constants use screaming snake case
    //must use type annotation
    //value must be a constant expression that can be determined at compile time
    //can be place outside of functions at global module scope
    //inline to compile time so they are fast
    //const WARP_FACTOR: f64: = 9.9;




}
