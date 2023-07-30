fn main() {
    let fox = RedFox::new();

    let is_fox = if fox.enemy {"is an enemy"}  else  {"is not an enemy"};
    println!("Hello fox. This fox {}", is_fox);
    println!("Hello fox. How old are you? Ans: {}", fox.life);
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