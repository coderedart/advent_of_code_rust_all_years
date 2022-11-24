const INPUT: &str = "bgvyzdsv";

fn main() {
    {
        let mut hasher = md5::Context::new();
        hasher.consume(INPUT);
        for num in 1..u32::MAX {
            let mut hasher = hasher.clone();
            hasher.consume(format!("{}", num));
            let h = hasher.compute().0;
            if h.starts_with(&[0, 0]) && h[2] < 16 {
                dbg!(num);
                break;
            }
        }
    }
    {
        let mut hasher = md5::Context::new();
        hasher.consume(INPUT);
        for num in 1..u32::MAX {
            let mut hasher = hasher.clone();
            hasher.consume(format!("{}", num));
            let h = hasher.compute().0;
            if h.starts_with(&[0, 0, 0]) {
                dbg!(num);
                break;
            }
        }
    }
}
