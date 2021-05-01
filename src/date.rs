#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gregorian(pub i32, pub i32, pub i32);

impl Gregorian {
    pub fn year(self) -> i32 {
        self.0
    }
    pub fn month(self) -> i32 {
        self.1
    }
    pub fn day(self) -> i32 {
        self.2
    }
}

impl std::fmt::Display for Gregorian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year(), self.month(), self.day(),)
    }
}

fn muldiv(var: i32, mul: i32, div: i32) -> i32 {
    use std::ops::Mul;
    var.mul(mul).div_euclid(div)
}

fn days_in_years(y: i32) -> i32 {
    muldiv(y, 1461, 4) - muldiv(y, 1, 100) + muldiv(y, 1, 400)
}

impl From<Gregorian> for i32 {
    fn from(Gregorian(y, m, d): Gregorian) -> i32 {
        let (y, m) = if m > 2 { (y, m + 1) } else { (y - 1, m + 13) };
        days_in_years(y) + muldiv(m, 153, 5) + d - 679004
    }
}

impl From<i32> for Gregorian {
    fn from(mjd: i32) -> Gregorian {
        let mut d = mjd + 678881;
        let mut y = muldiv(d, 400, 146097) + 1;
        y -= (days_in_years(y) > d) as i32;
        d -= days_in_years(y) - 31;
        let m = muldiv(d, 17, 520);
        d -= muldiv(m, 520, 17);
        if m > 10 {
            Gregorian(y + 1, m - 10, d)
        } else {
            Gregorian(y + 0, m + 2, d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for &(date, mjd) in &[
            (Gregorian(-1, 12, 31), -678942),
            (Gregorian(0, 1, 1), -678941),
            (Gregorian(0, 12, 31), -678576),
            (Gregorian(1, 1, 1), -678575),
            (Gregorian(1858, 11, 16), -1),
            (Gregorian(1858, 11, 17), 0),
            (Gregorian(1900, 1, 1), 15020),
            (Gregorian(1970, 1, 1), 40587),
            (Gregorian(2001, 1, 1), 5 * 146097 - 678575),
            (Gregorian(2020, 2, 2), 58881),
        ] {
            assert_eq!(date, Gregorian::from(mjd));
            assert_eq!(mjd, i32::from(date));
        }
        assert_eq!(146097, days_in_years(400));
    }
}
