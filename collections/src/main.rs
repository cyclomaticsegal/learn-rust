use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(2);
    v.push(4);
    v.push(6);
    v.push(8);

    let _x: Option<i32> = v.pop();

    //check length before using indexer
    //if out of bounds then rust will panic
    println!("{}", v[1]);

    v = vec![1,3,5,7,9];
    v.pop();
    v.pop();

   if v.len() > 2 {
    println!("{}", v[1]);
   } else {
    println!("nothing here");
   }

   let mut h: HashMap<u8, bool> = HashMap::new();
   h.insert(5, true);
   h.insert(6, false);

   let have_five = h.remove(&5).unwrap();

   println!("removed from hash map element where the key was 5 and the value was:{}", have_five);

}