When will the next leap second happen?
======================================

The IERS publishes a number of bulletins about UTC and leap seconds.

https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html

Bulletin C (twice a year) announces whether a leap second will or won't
happen.

Bulletin A (weekly) contains detailed predictions of how UT1 (earth
rotation angle) will differ from UTC for the next year.

When UT1-UTC gets close to 1 second, a leap second is announced to
keep them close together.

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

these scripts
-------------

Run `make`

This will download the first 16 weekly issues of Bulletin A for 2021
(with the script `bulletin-a-get.sh`)

It will compile `bulletin-a.c` which uses the formula to work out
which year UT1-UTC will increase past 0.5 seconds.

For each issue of Bulletin A it will run `bulletin-a.pl`, which parses
out the parameters of the formula and invokes `bulletin-a.c`.

> This is kind of a mess of scripts calling scripts because it was
> lazily hacked together without refactoring. It's just a quick code
> dump to show how I got my results.

output
------

The output from these scripts is:

	2040-01-01 MJD 66154 UT1-UTC +0.531552 +/- 0.189946 (0.000100)
	2038-01-01 MJD 65424 UT1-UTC +0.523150 +/- 0.174592 (0.000110)
	2035-01-01 MJD 64328 UT1-UTC +0.504966 +/- 0.150720 (0.000130)
	2034-01-01 MJD 63963 UT1-UTC +0.502595 +/- 0.142385 (0.000140)
	2033-01-01 MJD 63598 UT1-UTC +0.534394 +/- 0.133885 (0.000160)
	2032-01-01 MJD 63232 UT1-UTC +0.511222 +/- 0.125177 (0.000170)
	2032-01-01 MJD 63232 UT1-UTC +0.553342 +/- 0.125012 (0.000180)
	2031-01-01 MJD 62867 UT1-UTC +0.525831 +/- 0.116117 (0.000190)
	2031-01-01 MJD 62867 UT1-UTC +0.563821 +/- 0.115947 (0.000200)
	2030-01-01 MJD 62502 UT1-UTC +0.524680 +/- 0.106814 (0.000210)
	2030-01-01 MJD 62502 UT1-UTC +0.559040 +/- 0.106640 (0.000220)
	2029-01-01 MJD 62137 UT1-UTC +0.509369 +/- 0.097233 (0.000230)
	2029-01-01 MJD 62137 UT1-UTC +0.538099 +/- 0.097053 (0.000240)
	2029-01-01 MJD 62137 UT1-UTC +0.564089 +/- 0.096873 (0.000250)
	2029-01-01 MJD 62137 UT1-UTC +0.560739 +/- 0.096693 (0.000250)
	2029-01-01 MJD 62137 UT1-UTC +0.587419 +/- 0.096513 (0.000260)

licence
-------

> This code was written by Tony Finch <<dot@dotat.at>>  
> You may do anything with this. It has no warranty.  
> <https://creativecommons.org/publicdomain/zero/1.0/>  
> SPDX-License-Identifier: CC0-1.0
