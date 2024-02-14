use rand::prelude::*;

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn get_i32(max: i32) -> i32 {
    let r_f64 = random();
    (r_f64 * max as f64) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random() {
        println!("random {:?}", random());
        assert_eq!(true, true);
    }

    #[test]
    fn get_random_i32() {
        for i in 1..20 {
            println!("get_i32: {:?}", get_i32(10));
        }
        assert_eq!(true, true);
    }
}
