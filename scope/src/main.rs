fn main() {
    //BLOCK EXAMPLE 1
    //=============================
    // let x = 5;
    // {
    //     let y = 99;
    //     println!("{}, {}", x, y);
    // } // BLOCK ENDS HERE AND NO GC SO y is IMMEDIATELY DROPPED WHEN OUT OF SCOPE
    // println!("{}, {}", x, y); // PRODUCES AN ERROR AS y IS OUT OF SCOPE (DISCOVERED AT COMPILE TIME)
    //==============================

    //SHADOWING VARIABLES IN DIFFERENT SCOPES
    //==============================
    // let x = 5;
    // {
    //     let x = 99; //shadowed and only visible x within this scope
    //     println!("{}", x); //prints 99
    // }
    // println!("{}", x); //prints 5

    //SHADOWING VARIABLES IN SAME SCOPE
    //==============================
    // let mut x = 5;
    // let x = x; // x is now IMMUTABLE. REDEFINES X WITH DIFFERENT MUTABILITY
    // println!("{}", x);

    //SHADOW A VARIABLE TO DIFFERENT TYPE
    //===============================
    // let meme = "More cowbell!";
    // let meme = 32;
    // println!("{}", meme);
}
