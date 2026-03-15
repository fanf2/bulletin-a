set term qt size 2000,1000
set grid dashtype ". "

set xdata time
set timefmt "%Y-%m-%d"
set xtics format "%Y-%m-%d" time
set xtics 12 months
set mxtics 12
set ytics 100

set linetype 1 linewidth 2
set linetype 2 linewidth 2
set linetype 5 linecolor "black"

plot '< ./lod.pl <finals2000A.data.json' \
    using 1:2 with lines title "UT1-UTC" ,''\
    using 1:5 with lines title "LoD"     ,''\
    using 1:6 with lines title "dX"      ,''\
    using 1:7 with lines title "dY"      ,''\
    eq''? 0:0 with lines notitle

pause mouse close
