fn main() {
    println!("Hello, world!");
    second_func();
}

fn second_func() {
    println!("The second function");
    let args = with_args(3.3, 4.4);
    println!("{}", args.to_string());
}

fn with_args(qty: f64, oz: f64) -> f64 {
    //alt return can be {qty * oz}
    //return qty * oz;
    qty * oz
}
