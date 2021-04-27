#!/usr/bin/perl

use warnings;
use strict;

if (@ARGV != 1) {
	die "usage: bulletin-a.pl <file>\n";
}

my ($dut1, $rate, $mjd1, $mjd2);

while (<>) {
	if (m{^\s*UT1-UTC\s*=\s*
	      ([0-9.+-]+)\s*
	      ([+-])\s*([0-9.]+)\s*
	      [(]MJD\s*-\s*(\d+)[)]\s*
	      -\s*[(]UT2-UT1[)]\s*$}x) {
		$dut1 = $1;
		$rate = "$2$3";
		$mjd1 = $4;
	}
	if (m{  S t = 0.00025 \(MJD-(\d+)\)\*\*0.75\s*$}) {
		$mjd2 = $1;
	}
}

unless (defined $dut1 and defined $rate and defined $mjd1) {
	die "failed to parse UT1-UTC\n";
}
unless (defined $mjd2) {
	die "failed to parse S t\n";
}

system "./bulletin-a $dut1 $rate $mjd1 $mjd2";
