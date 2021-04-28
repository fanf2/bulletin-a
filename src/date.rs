use crate::euclid::Euclid;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MJD(Euclid);

impl From<i32> for MJD {
    fn from(i: i32) -> MJD {
        MJD(i.into())
    }
}

impl From<MJD> for i32 {
    fn from(d: MJD) -> i32 {
        d.0.into()
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gregorian {
    y: Euclid,
    m: Euclid,
    d: Euclid,
}

pub fn gregorian(y: i32, m: i32, d: i32) -> Gregorian {
    Gregorian { y: y.into(), m: m.into(), d: d.into() }
}

impl std::fmt::Debug for Gregorian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Gregorian({:04}-{:02}-{:02})",
            i32::from(self.y),
            i32::from(self.m),
            i32::from(self.d),
        )
    }
}

fn days_in_years(y: Euclid) -> Euclid {
    y * 1461 / 4 - y / 100 + y / 400
}

impl From<Gregorian> for MJD {
    fn from(g: Gregorian) -> MJD {
        let (y, m, d) = if g.m > 2.into() {
            (g.y, g.m + 1, g.d)
        } else {
            (g.y - 1, g.m + 13, g.d)
        };
        MJD(days_in_years(y) + m * 153 / 5 + d - 679004)
    }
}

impl From<MJD> for Gregorian {
    fn from(MJD(mut d): MJD) -> Gregorian {
        d += 678881;
        let mut y = d * 400 / 146097 + 1;
        if d < days_in_years(y) {
            y -= 1;
        }
        d -= days_in_years(y) - 31;
        let m = d * 17 / 520;
        d -= m * 520 / 17;
        if m > 10.into() {
            Gregorian { y: y + 1, m: m - 10, d }
        } else {
            Gregorian { y: y, m: m + 2, d: d }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        for (g, mjd) in &[
            (gregorian(-1, 12, 31), MJD::from(-678942)),
            (gregorian(0, 1, 1), MJD::from(-678941)),
            (gregorian(0, 12, 31), MJD::from(-678576)),
            (gregorian(1, 1, 1), MJD::from(-678575)),
            (gregorian(1858, 11, 16), MJD::from(-1)),
            (gregorian(1858, 11, 17), MJD::from(0)),
            (gregorian(1970, 1, 1), MJD::from(40587)),
            (gregorian(2020, 2, 2), MJD::from(58881)),
        ] {
            assert_eq!(*g, Gregorian::from(*mjd));
            assert_eq!(*mjd, MJD::from(*g));
        }
    }
}
