When will the next leap second happen?
======================================

> the results from this code are discussed in a twitter thread
> https://twitter.com/fanf/status/1386838657093586944

The IERS publishes a number of bulletins about UTC and leap seconds.

https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html

Bulletin C (twice a year) announces whether a leap second will or won't
happen.

Bulletin A (weekly, every Thursday) contains detailed predictions of
how UT1 (earth rotation angle) will differ from UTC for the next year.

When UT1-UTC grows significantly more than half a second, a leap
second is announced to keep them close together.

Historically, UT1-UTC has always decreased, and leap seconds have
always been positive. But at the moment, UT1-UTC is increasing. If
this continues long enough, there could be a negative leap second.

As well as a table of predictions for the next year, Bulletin A also
contains a simple formula that can be used for more long-term
projections. We can use this formua to estimate when the next leap
second might happen.

It isn't clear to me how far into the future it is sensible to use
this formula. The formula changes from week to week, so I thought it
would be informative to see how the prediction for the next leap
second changes in each issue of Bulletin A.

this program
------------

This repository contains a Rust program that downloads Bulletin A,
parses it, and calculates a forecast of when the next leap second
might happen.

If you type `cargo run` you should see output like:

		Finished dev [unoptimized + debuginfo] target(s) in 0.04s
		 Running `target/debug/bulletin-a`
	fetching https://datacenter.iers.org/data/6/bulletina-xxxiv-017.txt
	2021-04-29 -> 2028-06-30 (-) UT1-UTC +0.514 ± 0.092 s (lod -260 µs)

The first date is the date of the latest Bulletin A.

The second date is when the next leap second might happen. A row of
question marks indicates that there is no leap second in the
foreseeable future.

The `(+)` or `(-)` indicates whether it is predicted to be a positive
or negative leap second.

Then follow some numbers showing the forecast difference between UT1
and UTC at the time of the leap second, and the accuracy of the
forecast.

Finally is the length-of-day value used by the prediction formula,
given as microseconds different from 24 * 60 * 60 seconds. Until
recently this number has been positive.

options
-------

  * type `cargo run 50`

    to show forecasts from the last 50 issues of Bulletin A

	by default one is shown

  * type `cargo run 1 0.75`

	to use a threshold of 0.75 seconds when forecasting the next leap
    second; a leap second is predicted when UT1-UTC grows more than
    this threshold

	by default the threshold is 0.60 seconds

example output
--------------

You can see the transition from days slightly longer to slightly
shorter than 24 * 60 * 60 seconds with `cargo run 50 0.75`

	2020-05-21 -> 2028-06-30 (+) UT1-UTC -0.656 ± 0.100 s (lod +140 µs)
	2020-05-28 -> 2028-06-30 (+) UT1-UTC -0.658 ± 0.100 s (lod +140 µs)
	2020-06-04 -> 2028-06-30 (+) UT1-UTC -0.658 ± 0.100 s (lod +140 µs)
	2020-06-11 -> 2028-06-30 (+) UT1-UTC -0.656 ± 0.100 s (lod +140 µs)
	2020-06-18 -> 2029-06-30 (+) UT1-UTC -0.670 ± 0.109 s (lod +130 µs)
	2020-06-25 -> 2029-06-30 (+) UT1-UTC -0.668 ± 0.109 s (lod +130 µs)
	2020-07-02 -> 2029-06-30 (+) UT1-UTC -0.662 ± 0.108 s (lod +130 µs)
	2020-07-09 -> 2030-06-30 (+) UT1-UTC -0.670 ± 0.117 s (lod +120 µs)
	2020-07-16 -> 2031-06-30 (+) UT1-UTC -0.670 ± 0.126 s (lod +110 µs)
	2020-07-23 -> 2031-06-30 (+) UT1-UTC -0.668 ± 0.126 s (lod +110 µs)
	2020-07-30 -> 2031-06-30 (+) UT1-UTC -0.630 ± 0.125 s (lod +100 µs)
	2020-08-06 -> 2031-06-30 (+) UT1-UTC -0.632 ± 0.125 s (lod +100 µs)
	2020-08-13 -> 2032-06-30 (+) UT1-UTC -0.622 ± 0.134 s (lod +90 µs)
	2020-08-20 -> 2033-06-30 (+) UT1-UTC -0.649 ± 0.142 s (lod +90 µs)
	2020-08-27 -> 2034-06-30 (+) UT1-UTC -0.625 ± 0.150 s (lod +80 µs)
	2020-09-03 -> 2035-06-30 (+) UT1-UTC -0.595 ± 0.158 s (lod +70 µs)
	2020-09-10 -> 2036-06-30 (+) UT1-UTC -0.617 ± 0.166 s (lod +70 µs)
	2020-09-17 -> 2037-06-30 (+) UT1-UTC -0.580 ± 0.173 s (lod +60 µs)
	2020-09-24 -> 2038-06-30 (+) UT1-UTC -0.597 ± 0.181 s (lod +60 µs)
	2020-10-01 -> 2040-06-30 (+) UT1-UTC -0.561 ± 0.196 s (lod +50 µs)
	2020-10-08 -> 2044-06-30 (+) UT1-UTC -0.543 ± 0.225 s (lod +40 µs)
	2020-10-15 -> 2048-06-30 (+) UT1-UTC -0.499 ± 0.252 s (lod +30 µs)
	2020-10-22 -> 2056-06-30 (+) UT1-UTC -0.457 ± 0.305 s (lod +20 µs)
	2020-10-29 -> 2056-06-30 (+) UT1-UTC -0.456 ± 0.305 s (lod +20 µs)
	2020-11-05 -> 2856-06-30 (+) UT1-UTC -3.246 ± 3.246 s (lod +10 µs)
	2020-11-12 -> ????-??-?? (?)
	2020-11-19 -> ????-??-?? (?)
	2020-11-26 -> 2158-12-31 (-) UT1-UTC +0.843 ± 0.841 s (lod -20 µs)
	2020-12-03 -> 2073-12-31 (-) UT1-UTC +0.416 ± 0.411 s (lod -30 µs)
	2020-12-10 -> 2060-12-31 (-) UT1-UTC +0.419 ± 0.333 s (lod -40 µs)
	2020-12-17 -> 2054-12-31 (-) UT1-UTC +0.457 ± 0.294 s (lod -50 µs)
	2020-12-24 -> 2047-12-31 (-) UT1-UTC +0.528 ± 0.248 s (lod -70 µs)
	2020-12-31 -> 2044-12-31 (-) UT1-UTC +0.539 ± 0.226 s (lod -80 µs)
	2021-01-07 -> 2040-12-31 (-) UT1-UTC +0.568 ± 0.197 s (lod -100 µs)
	2021-01-14 -> 2039-12-31 (-) UT1-UTC +0.604 ± 0.190 s (lod -110 µs)
	2021-01-21 -> 2036-12-31 (-) UT1-UTC +0.600 ± 0.167 s (lod -130 µs)
	2021-01-28 -> 2035-12-31 (-) UT1-UTC +0.605 ± 0.159 s (lod -140 µs)
	2021-02-04 -> 2034-12-31 (-) UT1-UTC +0.651 ± 0.150 s (lod -160 µs)
	2021-02-11 -> 2033-12-31 (-) UT1-UTC +0.635 ± 0.142 s (lod -170 µs)
	2021-02-18 -> 2032-12-31 (-) UT1-UTC +0.619 ± 0.134 s (lod -180 µs)
	2021-02-25 -> 2032-12-31 (-) UT1-UTC +0.665 ± 0.133 s (lod -190 µs)
	2021-03-04 -> 2031-12-31 (-) UT1-UTC +0.637 ± 0.125 s (lod -200 µs)
	2021-03-11 -> 2031-12-31 (-) UT1-UTC +0.678 ± 0.124 s (lod -210 µs)
	2021-03-18 -> 2030-12-31 (-) UT1-UTC +0.639 ± 0.116 s (lod -220 µs)
	2021-03-25 -> 2030-12-31 (-) UT1-UTC +0.677 ± 0.115 s (lod -230 µs)
	2021-04-01 -> 2030-06-30 (-) UT1-UTC +0.644 ± 0.111 s (lod -240 µs)
	2021-04-08 -> 2029-12-31 (-) UT1-UTC +0.655 ± 0.106 s (lod -250 µs)
	2021-04-15 -> 2029-12-31 (-) UT1-UTC +0.652 ± 0.106 s (lod -250 µs)
	2021-04-22 -> 2029-12-31 (-) UT1-UTC +0.682 ± 0.106 s (lod -260 µs)
	2021-04-29 -> 2029-12-31 (-) UT1-UTC +0.681 ± 0.106 s (lod -260 µs)

licence
-------

> This code was written by Tony Finch <<dot@dotat.at>>  
> You may do anything with this. It has no warranty.  
> <https://creativecommons.org/publicdomain/zero/1.0/>  
> SPDX-License-Identifier: CC0-1.0
