set term qt size 2000,1000
set grid dashtype ". "
set linetype 1 linewidth 2
set linetype 2 linewidth 2

set xdata time
set timefmt "%Y-%m-%d"
set xrange ["2006-06-01":"2026-06-01"]
set xtics format "%Y-%m-%d" time
set xtics 12 months
set mxtics 4
set ytics 100

# ./lod.pl <finals2000A.all.json >lod

plot 'lod' using 1:2 with lines title "UT1-UTC" \
   , ''    using 1:6 with lines title "LoD" \
   , 0               with lines notitle

pause mouse close
