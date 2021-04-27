#include <math.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

typedef unsigned int uint;

static uint
iso8601_mjd(uint y, uint m, uint d) {
        if(m > 2) { m += 1; } else { m += 13; y -= 1; }
        return(y*1461/4 - y/100 + y/400 + m*153/5 + d - 679004);
}

// https://gssc.esa.int/navipedia/index.php/Transformations_between_Time_Systems

static double
besselian_year(uint mjd) {
	return(2000.0 + (mjd - 51544.03) / 365.2422);
}

static double
ut2_ut1(uint mjd) {
	double T = besselian_year(mjd);
	double tT = 2 * M_PI * T;
	return(+ 0.022 * sin(tT)
	       - 0.012 * cos(tT)
	       - 0.006 * sin(2*tT)
	       + 0.007 * cos(2*tT));
}

static double
ut1_utc_2021_04_22(uint mjd) {
	return(-0.1462 + 0.00026 * (mjd - 59334.0) - ut2_ut1(mjd));
}

static double
accuracy_2021_04_22(uint mjd) {
	return(0.00025 * pow(mjd - 59326.0, 0.75));
}

static void
usage(void) {
	fprintf(stderr, "usage: bulletin-a <YYYY-MM-DD>\n");
	exit(1);
}

int main(int argc, char *argv[]) {

	if(argc != 2)
		usage();

	uint y, m, d;
	int n;
	int r = sscanf(argv[1], "%u-%u-%u%n", &y, &m, &d, &n);
	if(r != 3 || argv[1][n] != '\0')
		usage();

	uint mjd = iso8601_mjd(y, m, d);
	printf("%04u-%02u-%02u MJD %u UT1-UTC %+f +/- %f\n",
	       y, m, d, mjd,
	       ut1_utc_2021_04_22(mjd),
	       accuracy_2021_04_22(mjd));

	exit(0);
}
