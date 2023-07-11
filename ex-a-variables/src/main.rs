const STARTING_MISSILE:i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
    //let mut missiles:i32 = STARTING_MISSILE;
    //let ready:i32 = READY_AMOUNT;
    let (missiles, ready):(i32, i32) = (STARTING_MISSILE, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    //missiles = missiles - ready;
    //println!("{} missiles left", missiles);
    println!("{}", missiles - ready);
}
