mod date;
mod euclid;
mod roman;

use anyhow::{anyhow, Context, Result};
use date::*;
use roman::Roman;
use std::io::Read;

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

fn get_bulletin_a(issue: i32) -> Result<(Gregorian, i32, String)> {
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
    // to work more easily with nom
    Ok((date, week, String::from_utf8(data)?))
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Prediction {
    dut1: f64,
    lod: f64,
    mjd: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Precision {
    base: f64,
    pow: f64,
    mjd: i32,
}

type BulletinA = (Prediction, Precision);

type VerboseResult<'a, O> =
    nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

fn parse_bulletin_a<'a>(bula: &'a str) -> VerboseResult<'a, BulletinA> {
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
        BulA(BulletinA),
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
            (Pred(pred), Prec(prec)) => BulA((pred, prec)),
            _ => Clash("multiple equations"),
        })),
        |t| match t {
            BulA(param) => Ok(param),
            Clash(err) => Err(err),
            _ => Err("missing equation"),
        },
    );

    parse(bula)
}

fn main() -> Result<()> {
    // issue 4 is missing so skip a few
    for issue in 5..851 {
        let (date, week, bula) = get_bulletin_a(issue)?;
        let (_, param) =
            parse_bulletin_a(&bula).map_err(|e| anyhow!("{}", e))?;
        eprintln!(
            "{}  {:03}    DUT1 = {:+.3} s    lod = {:+.0} µs",
            date,
            week,
            param.0.dut1,
            param.0.lod * 1000000.0
        );
    }
    Ok(())
}
