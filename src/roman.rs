#[derive(Copy, Clone)]
pub struct Roman(pub usize);

impl std::fmt::Debug for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // we'll just panic if the number is too large
        let n = self.0;
        let m = n / 1000;
        let c = (n / 100) % 10;
        let x = (n / 10) % 10;
        let i = n % 10;
        write!(
            f,
            "{}{}{}{}",
            ["", "M", "MM", "MMM"][m],
            ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"][c],
            ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"][x],
            ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"][i],
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!("MMCDLXVIII", format!("{:?}", super::Roman(2468)));
    }
}
