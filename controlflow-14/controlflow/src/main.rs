fn main() {
    
    let num = 2;
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };
    println!("{}", msg);

    let a:bool = true;
    let b = "b";
    let c = "c";
    let msg = if a {b} else {c};

    println!("{}", msg);
}
