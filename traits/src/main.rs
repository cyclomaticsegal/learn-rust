fn main() {
    let fox = RedFox::new();

    let is_fox = if fox.enemy {"is an enemy"}  else  {"is not an enemy"};
    println!("Hello fox. This fox {}", is_fox);
    println!("Hello fox. How old are you? Ans: {}", fox.life);

    print_noise(5_u8);

    //reverse the next two lines gives an error due to 
    //reference to fox being borrowed by the print_noise fn and then destroy
    //when it goes out of scope of the fn block
    println!("{}", fox.get_noise());

    print_noise(fox);
    
}


struct RedFox {
    enemy: bool,
    life: u8,
}

impl RedFox {
    /// Creates a new [`RedFox`].
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
}

impl Default for RedFox {
    fn default() -> Self {
        Self::new()
    }
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {"meow"}
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {"BYTE!"}
}