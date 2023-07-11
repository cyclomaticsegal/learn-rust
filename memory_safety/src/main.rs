fn main() {
    //wont work because enigma not initialized
    //let enigma: i32;
    //println!("enigma is {}", enigma); //error because not 

    // let enigma: i32;
    // if true {
    //     enigma = 42;
    // }
    // println!("enigma is {}", enigma); //error wont compile either because cant tell if conditional will evaluate

    let enigma: i32;
    if true {
        enigma = 42;
    }else {
        enigma = 5;
    }
    println!("enigma is {}", enigma); //works because compiler can guarantee enigma will have a value
}
