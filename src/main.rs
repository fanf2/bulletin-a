mod date;
mod euclid;
mod roman;

use anyhow::{anyhow, Context, Result};
use date::*;
use roman::Roman;
use std::io::Read;

fn ut2_ut1(mjd: i32) -> f64 {
    // https://gssc.esa.int/navipedia/index.php/Transformations_between_Time_Systems
    let besselian_year = 2000.0 + (mjd as f64 - 51544.03) / 365.2422;
    let t = besselian_year * std::f64::consts::TAU;
    let tt = 2.0 * t;
    0.022 * t.sin() - 0.012 * t.cos() - 0.006 * tt.sin() + 0.007 * tt.cos()
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Prediction {
    dut1: f64,
    lod: f64,
    mjd: i32,
}

impl Prediction {
    fn at(self, mjd: i32) -> f64 {
        let days = (mjd - self.mjd) as f64;
        self.dut1 - self.lod * days - ut2_ut1(mjd)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Precision {
    base: f64,
    pow: f64,
    mjd: i32,
}

impl Precision {
    fn at(self, mjd: i32) -> f64 {
        let days = (mjd - self.mjd) as f64;
        self.base * days.powf(self.pow)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BulletinA {
    date: Gregorian,
    pred: Prediction,
    prec: Precision,
}

// recent leap seconds
//
// 2005-12-31 23:59:60
// 2008-12-31 23:59:60
// 2012-06-30 23:59:60
// 2015-06-30 23:59:60
// 2016-12-31 23:59:60

// silly thing so that fetching a URL looks like reading a file
struct URL(String);

impl URL {
    fn read_to_end(&self, data: &mut Vec<u8>) -> Result<()> {
        let mut ua = curl::easy::Easy::new();
        ua.useragent("fanf/1.0")?;
        ua.fail_on_error(true)?;
        ua.url(&self.0)?;
        let mut xfer = ua.transfer();
        xfer.write_function(|chunk| {
            data.extend_from_slice(chunk);
            Ok(chunk.len())
        })?;
        xfer.perform()?;
        Ok(())
    }
}

fn get_bulletin_a(issue: i32) -> Result<(Gregorian, String)> {
    // no bulletin issued on 2009-01-01 which would have been 208
    // first available issue
    let zero =
        i32::from(gregorian(2005, 1, 6)) + if issue >= 208 { 7 } else { 0 };
    // published weekly
    let date = Gregorian::from(zero + issue * 7);
    let year = date.year();
    let volume = Roman(year - 1987);
    // volume number is week within year
    let janus = i32::from(gregorian(year, 1, 1));
    let week = (i32::from(date) - janus) / 7 + if year == 2009 { 0 } else { 1 };
    let weeks = format!("{:03}", week);
    let prefix = "https://datacenter.iers.org/data/6/bulletina";
    let url = URL(format!("{}-{:?}-{}.txt", prefix, volume, weeks));
    // local cache
    let dir = format!("bula/{}", year);
    let file = format!("{}/{}", dir, weeks);
    std::fs::create_dir_all(dir)?;
    let mut data = Vec::new();
    if let Ok(mut fh) = std::fs::File::open(&file) {
        fh.read_to_end(&mut data)
            .with_context(|| format!("failed to read {}", file))?;
    } else {
        eprintln!("fetching {}", &url.0);
        url.read_to_end(&mut data)
            .with_context(|| format!("failed to get {}", url.0))?;
        std::fs::write(&file, &data)?;
    }
    // strings work more easily with nom
    Ok((date, String::from_utf8(data)?))
}

type ParseResult<'a> =
    nom::IResult<&'a str, BulletinA, nom::error::VerboseError<&'a str>>;

fn parse_bulletin_a<'a>(date: Gregorian, bula: &'a str) -> ParseResult<'a> {
    use nom::branch::*;
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::combinator::*;
    use nom::multi::*;
    use nom::number::complete::*;
    use nom::sequence::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum State {
        Skip,
        Pred(Prediction),
        Prec(Precision),
        BulA(Gregorian, Prediction, Precision),
        Clash(&'static str),
    }
    use State::*;

    let tail = || tuple((space0, line_ending));

    // UT1-UTC = -0.NNNNN +- 0.000NNN (MJD - NNNNN) - (UT2-UT1)
    let pred = map(
        tuple((
            preceded(tuple((space0, tag("UT1-UTC ="), space0)), double),
            preceded(space0, one_of("+-")),
            preceded(space0, double),
            delimited(
                tuple((space0, tag("(MJD - "))),
                double,
                tuple((space0, tag(") - (UT2-UT1)"), tail())),
            ),
        )),
        |t| {
            Pred(Prediction {
                dut1: t.0,
                lod: match t.1 {
                    '-' => t.2,
                    _ => -t.2,
                },
                mjd: t.3 as i32,
            })
        },
    );

    // S x,y = 0.00068 (MJD-59326)**0.80   S t = 0.00025 (MJD-59326)**0.75
    let precision = |var| {
        map(
            tuple((
                preceded(
                    tuple((space0, tag("S "), tag(var), tag(" = "))),
                    double,
                ),
                preceded(tag(" (MJD-"), double),
                preceded(tag(")**"), double),
            )),
            |t| Prec(Precision { base: t.0, mjd: t.1 as i32, pow: t.2 }),
        )
    };
    let prec = delimited(precision("x,y"), precision("t"), tail());

    let skip = value(Skip, pair(not_line_ending, line_ending));

    let line = alt((pred, prec, skip));

    let mut parse = map_res(
        complete(fold_many0(line, Skip, |acc, item| match (acc, item) {
            (acc, Skip) => acc,
            (Skip, Pred(pred)) => Pred(pred),
            (Pred(pred), Prec(prec)) => BulA(date, pred, prec),
            _ => Clash("multiple equations"),
        })),
        |t| match t {
            BulA(date, pred, prec) => Ok(BulletinA { date, pred, prec }),
            Clash(err) => Err(err),
            _ => Err("missing equation"),
        },
    );

    parse(bula)
}

fn bulletin_a(issue: i32) -> Result<BulletinA> {
    let (date, bula) = get_bulletin_a(issue)?;
    let (_, param) =
        parse_bulletin_a(date, &bula).map_err(|e| anyhow!("{}", e))?;
    Ok(param)
}

fn main() -> Result<()> {
    let param = bulletin_a(849)?;
    eprintln!("predictions as of {}", param.date);
    for year in 2022..=2030 {
        let greg = gregorian(year, 1, 1);
        let mjd = i32::from(greg);
        eprintln!(
            "{} MJD {} UT1-UTC {:+.6} ± {:.6} s",
            greg,
            mjd,
            param.pred.at(mjd),
            param.prec.at(mjd)
        );
    }
    return Ok(());
    // issue 4 is missing so skip a few
    for issue in 5..851 {
        let param = bulletin_a(issue)?;
        eprintln!(
            "{}    DUT1 = {:+.3} s    lod = {:+.0} µs",
            param.date,
            param.pred.dut1,
            param.pred.lod * 1000000.0
        );
    }
    Ok(())
}
