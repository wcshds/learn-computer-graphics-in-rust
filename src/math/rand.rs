#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

    #[test]
    fn dice_roll_out() {
        let mut rng = thread_rng();
    
        println!("Dice roll: {}", rng.gen_range(1..=6));
    }
}