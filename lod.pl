#!/usr/bin/perl

use warnings;
use strict;
use v5.30;

use Data::Dumper;
use JSON;
use List::Util qw{ sum };

# metadata https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop
# eg https://datacenter.iers.org/data/json/finals2000A.data.json
# eg https://datacenter.iers.org/data/json/finals2000A.all.json

# slurp
undef $/;
my $eop = decode_json <STDIN>;

my @tk = qw{ dateYear dateMonth dateDay };
my @dk = qw{ UT1-UTC LOD };

my @lodm;
my @lodmm;
my @lody;
my @lodyy;

for my $i (@{ $eop->{EOP}->{data}->{timeSeries} }) {
	my $t = $i->{time};
	my $date = join '-', $t->@{@tk};
	my $lod = $i->{dataEOP}->{UT}->[0]->{LOD};
	my $dut = $i->{dataEOP}->{UT}->[0]->{'UT1-UTC'};
	last if $lod eq '';
	$lod *= 1000;
	$dut *= 1000;
	push @lodm,  $lod; shift @lodm  if @lodm  > 30;
	push @lodmm, $lod; shift @lodmm if @lodmm > 30 * 3;
	push @lody,  $lod; shift @lody  if @lody  > 365;
	push @lodyy, $lod; shift @lodyy if @lodyy > 365 * 4;
	printf "%s %+4.0f %+5.0f %+5.0f %+5.0f %+5.0f %+5.0f\n",
	    $date, $dut,
	    $lod,
	    sum(@lodm) / @lodm,
	    sum(@lodmm) / @lodmm,
	    sum(@lody) / @lody,
	    sum(@lodyy) / @lodyy
	    if $t->{dateDay} eq '01';
}

say "#   date   dutc   lod   lod   lod   lod   lod";
say "#            ms    µs    µs    µs    µs    µs";
say "#                 day   1mo   4mo    1y    4y";
