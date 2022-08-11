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
shorter than 24 * 60 * 60 seconds with `cargo run -- 100 0.6`

    2020-09-17 -> 2032-06-30 (+) UT1-UTC -0.470 ± 0.133 s (lod +60 µs)
    2020-09-24 -> 2033-06-30 (+) UT1-UTC -0.487 ± 0.141 s (lod +60 µs)
    2020-10-01 -> 2034-06-30 (+) UT1-UTC -0.452 ± 0.149 s (lod +50 µs)
    2020-10-08 -> 2037-06-30 (+) UT1-UTC -0.441 ± 0.173 s (lod +40 µs)
    2020-10-15 -> 2040-06-30 (+) UT1-UTC -0.412 ± 0.195 s (lod +30 µs)
    2020-10-22 -> 2045-06-30 (+) UT1-UTC -0.377 ± 0.231 s (lod +20 µs)
    2020-10-29 -> 2045-06-30 (+) UT1-UTC -0.376 ± 0.231 s (lod +20 µs)
    2020-11-05 -> 2054-06-30 (+) UT1-UTC -0.318 ± 0.292 s (lod +10 µs)
    2020-11-12 -> ????-??-?? (?)
    2020-11-19 -> ????-??-?? (?)
    2020-11-26 -> 2158-12-31 (-) UT1-UTC +0.843 ± 0.841 s (lod -20 µs)
    2020-12-03 -> 2073-12-31 (-) UT1-UTC +0.416 ± 0.411 s (lod -30 µs)
    2020-12-10 -> 2053-12-31 (-) UT1-UTC +0.317 ± 0.288 s (lod -40 µs)
    2020-12-17 -> 2048-12-31 (-) UT1-UTC +0.347 ± 0.255 s (lod -50 µs)
    2020-12-24 -> 2042-12-31 (-) UT1-UTC +0.400 ± 0.212 s (lod -70 µs)
    2020-12-31 -> 2040-12-31 (-) UT1-UTC +0.422 ± 0.198 s (lod -80 µs)
    2021-01-07 -> 2037-12-31 (-) UT1-UTC +0.459 ± 0.175 s (lod -100 µs)
    2021-01-14 -> 2035-12-31 (-) UT1-UTC +0.443 ± 0.159 s (lod -110 µs)
    2021-01-21 -> 2033-12-31 (-) UT1-UTC +0.458 ± 0.143 s (lod -130 µs)
    2021-01-28 -> 2033-12-31 (-) UT1-UTC +0.503 ± 0.142 s (lod -140 µs)
    2021-02-04 -> 2031-12-31 (-) UT1-UTC +0.476 ± 0.125 s (lod -160 µs)
    2021-02-11 -> 2031-12-31 (-) UT1-UTC +0.511 ± 0.125 s (lod -170 µs)
    2021-02-18 -> 2030-12-31 (-) UT1-UTC +0.488 ± 0.116 s (lod -180 µs)
    2021-02-25 -> 2030-12-31 (-) UT1-UTC +0.526 ± 0.116 s (lod -190 µs)
    2021-03-04 -> 2030-06-30 (-) UT1-UTC +0.502 ± 0.111 s (lod -200 µs)
    2021-03-11 -> 2029-12-31 (-) UT1-UTC +0.525 ± 0.107 s (lod -210 µs)
    2021-03-18 -> 2029-12-31 (-) UT1-UTC +0.559 ± 0.107 s (lod -220 µs)
    2021-03-25 -> 2028-12-31 (-) UT1-UTC +0.509 ± 0.097 s (lod -230 µs)
    2021-04-01 -> 2028-12-31 (-) UT1-UTC +0.538 ± 0.097 s (lod -240 µs)
    2021-04-08 -> 2028-12-31 (-) UT1-UTC +0.564 ± 0.097 s (lod -250 µs)
    2021-04-15 -> 2028-12-31 (-) UT1-UTC +0.561 ± 0.097 s (lod -250 µs)
    2021-04-22 -> 2028-06-30 (-) UT1-UTC +0.515 ± 0.092 s (lod -260 µs)
    2021-04-29 -> 2028-06-30 (-) UT1-UTC +0.514 ± 0.092 s (lod -260 µs)
    2021-05-06 -> 2028-06-30 (-) UT1-UTC +0.512 ± 0.091 s (lod -260 µs)
    2021-05-13 -> 2028-12-31 (-) UT1-UTC +0.578 ± 0.096 s (lod -260 µs)
    2021-05-20 -> 2028-12-31 (-) UT1-UTC +0.573 ± 0.096 s (lod -260 µs)
    2021-05-27 -> 2028-12-31 (-) UT1-UTC +0.569 ± 0.096 s (lod -260 µs)
    2021-06-03 -> 2028-12-31 (-) UT1-UTC +0.539 ± 0.095 s (lod -250 µs)
    2021-06-10 -> 2028-12-31 (-) UT1-UTC +0.536 ± 0.095 s (lod -250 µs)
    2021-06-17 -> 2028-12-31 (-) UT1-UTC +0.534 ± 0.095 s (lod -250 µs)
    2021-06-24 -> 2028-12-31 (-) UT1-UTC +0.534 ± 0.095 s (lod -250 µs)
    2021-07-01 -> 2028-12-31 (-) UT1-UTC +0.534 ± 0.095 s (lod -250 µs)
    2021-07-08 -> 2028-12-31 (-) UT1-UTC +0.531 ± 0.094 s (lod -250 µs)
    2021-07-15 -> 2028-12-31 (-) UT1-UTC +0.527 ± 0.094 s (lod -250 µs)
    2021-07-22 -> 2028-12-31 (-) UT1-UTC +0.522 ± 0.094 s (lod -250 µs)
    2021-07-29 -> 2029-06-30 (-) UT1-UTC +0.509 ± 0.099 s (lod -240 µs)
    2021-08-05 -> 2029-06-30 (-) UT1-UTC +0.505 ± 0.098 s (lod -240 µs)
    2021-08-12 -> 2029-06-30 (-) UT1-UTC +0.505 ± 0.098 s (lod -240 µs)
    2021-08-19 -> 2029-12-31 (-) UT1-UTC +0.546 ± 0.103 s (lod -230 µs)
    2021-08-26 -> 2029-12-31 (-) UT1-UTC +0.548 ± 0.103 s (lod -230 µs)
    2021-09-02 -> 2029-12-31 (-) UT1-UTC +0.550 ± 0.102 s (lod -230 µs)
    2021-09-09 -> 2029-12-31 (-) UT1-UTC +0.551 ± 0.102 s (lod -230 µs)
    2021-09-16 -> 2029-12-31 (-) UT1-UTC +0.554 ± 0.102 s (lod -230 µs)
    2021-09-23 -> 2029-12-31 (-) UT1-UTC +0.556 ± 0.102 s (lod -230 µs)
    2021-09-30 -> 2029-12-31 (-) UT1-UTC +0.558 ± 0.102 s (lod -230 µs)
    2021-10-07 -> 2029-12-31 (-) UT1-UTC +0.560 ± 0.102 s (lod -230 µs)
    2021-10-14 -> 2029-12-31 (-) UT1-UTC +0.532 ± 0.101 s (lod -220 µs)
    2021-10-21 -> 2029-12-31 (-) UT1-UTC +0.533 ± 0.101 s (lod -220 µs)
    2021-10-28 -> 2029-12-31 (-) UT1-UTC +0.534 ± 0.101 s (lod -220 µs)
    2021-11-04 -> 2029-12-31 (-) UT1-UTC +0.535 ± 0.101 s (lod -220 µs)
    2021-11-11 -> 2029-12-31 (-) UT1-UTC +0.507 ± 0.101 s (lod -210 µs)
    2021-11-18 -> 2029-12-31 (-) UT1-UTC +0.509 ± 0.100 s (lod -210 µs)
    2021-11-25 -> 2029-12-31 (-) UT1-UTC +0.510 ± 0.100 s (lod -210 µs)
    2021-12-02 -> 2029-12-31 (-) UT1-UTC +0.510 ± 0.100 s (lod -210 µs)
    2021-12-09 -> 2029-12-31 (-) UT1-UTC +0.507 ± 0.100 s (lod -210 µs)
    2021-12-16 -> 2029-12-31 (-) UT1-UTC +0.503 ± 0.100 s (lod -210 µs)
    2021-12-23 -> 2030-12-31 (-) UT1-UTC +0.545 ± 0.109 s (lod -200 µs)
    2021-12-30 -> 2030-12-31 (-) UT1-UTC +0.542 ± 0.109 s (lod -200 µs)
    2022-01-06 -> 2030-12-31 (-) UT1-UTC +0.539 ± 0.108 s (lod -200 µs)
    2022-01-13 -> 2030-12-31 (-) UT1-UTC +0.538 ± 0.108 s (lod -200 µs)
    2022-01-20 -> 2030-12-31 (-) UT1-UTC +0.539 ± 0.108 s (lod -200 µs)
    2022-01-27 -> 2030-12-31 (-) UT1-UTC +0.542 ± 0.108 s (lod -200 µs)
    2022-02-03 -> 2030-12-31 (-) UT1-UTC +0.546 ± 0.108 s (lod -200 µs)
    2022-02-10 -> 2030-12-31 (-) UT1-UTC +0.548 ± 0.108 s (lod -200 µs)
    2022-02-17 -> 2029-12-31 (-) UT1-UTC +0.508 ± 0.098 s (lod -210 µs)
    2022-02-24 -> 2029-12-31 (-) UT1-UTC +0.509 ± 0.098 s (lod -210 µs)
    2022-03-03 -> 2029-12-31 (-) UT1-UTC +0.515 ± 0.098 s (lod -210 µs)
    2022-03-10 -> 2029-12-31 (-) UT1-UTC +0.549 ± 0.098 s (lod -220 µs)
    2022-03-17 -> 2029-06-30 (-) UT1-UTC +0.516 ± 0.093 s (lod -230 µs)
    2022-03-24 -> 2029-06-30 (-) UT1-UTC +0.521 ± 0.092 s (lod -230 µs)
    2022-03-31 -> 2028-12-31 (-) UT1-UTC +0.533 ± 0.088 s (lod -240 µs)
    2022-04-07 -> 2028-12-31 (-) UT1-UTC +0.561 ± 0.087 s (lod -250 µs)
    2022-04-14 -> 2028-12-31 (-) UT1-UTC +0.587 ± 0.087 s (lod -260 µs)
    2022-04-21 -> 2028-12-31 (-) UT1-UTC +0.590 ± 0.087 s (lod -260 µs)
    2022-04-28 -> 2028-06-30 (-) UT1-UTC +0.540 ± 0.082 s (lod -270 µs)
    2022-05-05 -> 2027-12-31 (-) UT1-UTC +0.533 ± 0.077 s (lod -280 µs)
    2022-05-12 -> 2027-12-31 (-) UT1-UTC +0.528 ± 0.076 s (lod -280 µs)
    2022-05-19 -> 2028-06-30 (-) UT1-UTC +0.549 ± 0.081 s (lod -280 µs)
    2022-05-26 -> 2027-12-31 (-) UT1-UTC +0.541 ± 0.076 s (lod -290 µs)
    2022-06-02 -> 2027-12-31 (-) UT1-UTC +0.542 ± 0.076 s (lod -290 µs)
    2022-06-09 -> 2027-12-31 (-) UT1-UTC +0.562 ± 0.076 s (lod -300 µs)
    2022-06-16 -> 2027-12-31 (-) UT1-UTC +0.562 ± 0.075 s (lod -300 µs)
    2022-06-23 -> 2027-12-31 (-) UT1-UTC +0.564 ± 0.075 s (lod -300 µs)
    2022-06-30 -> 2027-12-31 (-) UT1-UTC +0.580 ± 0.075 s (lod -310 µs)
    2022-07-07 -> 2027-12-31 (-) UT1-UTC +0.598 ± 0.075 s (lod -320 µs)
    2022-07-14 -> 2027-12-31 (-) UT1-UTC +0.596 ± 0.075 s (lod -320 µs)
    2022-07-21 -> 2027-12-31 (-) UT1-UTC +0.612 ± 0.074 s (lod -330 µs)
    2022-07-28 -> 2027-12-31 (-) UT1-UTC +0.608 ± 0.074 s (lod -330 µs)
    2022-08-04 -> 2027-12-31 (-) UT1-UTC +0.605 ± 0.074 s (lod -330 µs)
    2022-08-11 -> 2027-06-30 (-) UT1-UTC +0.535 ± 0.069 s (lod -340 µs)

licence
-------

> This code was written by Tony Finch <<dot@dotat.at>>  
> You may do anything with this. It has no warranty.  
> <https://creativecommons.org/publicdomain/zero/1.0/>  
> SPDX-License-Identifier: CC0-1.0
