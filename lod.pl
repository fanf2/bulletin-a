#!/usr/bin/perl

use warnings;
use strict;
use v5.30;

use JSON;
use List::Util qw{ sum };

# metadata https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop
# eg https://datacenter.iers.org/data/json/finals2000A.data.json
# eg https://datacenter.iers.org/data/json/finals2000A.all.json

# slurp
undef $/;
my $eop = decode_json <STDIN>;

my @tk = qw{ dateYear dateMonth dateDay };

my $bulA = 0;
my $bulB = 1;

my @lodm;
my @lody;

for my $i (@{ $eop->{EOP}->{data}->{timeSeries} }) {
	my $t = $i->{time};
	my $date = join '-', $t->@{@tk};

	my $lod = $i->{dataEOP}->{UT}->[$bulA]->{LOD};
	my $dut = $i->{dataEOP}->{UT}->[$bulA]->{'UT1-UTC'};
	my $dX = $i->{dataEOP}->{nutation}->[$bulA]->{dX};
	my $dY = $i->{dataEOP}->{nutation}->[$bulA]->{dY};
	last unless $lod and $dut and $dX and $dY;

	$lod *= 1000;
	$dut *= 1000;
	$dX *= 1000;
	$dY *= 1000;

	push @lodm, $lod; shift @lodm if @lodm > 30;
	push @lody, $lod; shift @lody if @lody > 300;

	printf "%s %+4.0f %+5.0f %+5.0f %+5.0f %+5.0f %+5.0f\n",
	    $date, $dut, $lod,
	    sum(@lodm) / @lodm,
	    sum(@lody) / @lody,
	    $dX, $dY
	    if $date gt '2006-06-01';
}
