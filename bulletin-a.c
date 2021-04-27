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
ut1_utc(double dut1, double rate, uint mjd1, uint mjd3) {
	return(dut1 + rate * (mjd3 - mjd1) - ut2_ut1(mjd3));
}

static double
accuracy(uint mjd2, uint mjd3) {
	return(0.00025 * pow((double)(mjd3 - mjd2), 0.75));
}

static void
usage(const char *err) {
	fprintf(stderr, "error: %s\n", err);
	fprintf(stderr, "usage: "
		"bulletin-a <dut1> <rate> <mjd1> <mjd2> <YYYY-MM-DD>\n");
	exit(1);
}

int main(int argc, char *argv[]) {

	if(argc != 5)
		usage("incorrect number of arguments");

	double dut1 = atof(argv[1]);
	double rate = atof(argv[2]);
	uint mjd1 = (uint)atoi(argv[3]);
	uint mjd2 = (uint)atoi(argv[4]);

	uint y = 2022, m = 1, d = 1;

	for (;;) {
		uint mjd3 = iso8601_mjd(y, m, d);
		double u = ut1_utc(dut1, rate, mjd1, mjd3);
		double a = accuracy(mjd2, mjd3);
		if (u > 0.5) {
			printf("%04u-%02u-%02u MJD %u UT1-UTC %+f +/- %f (%f)\n",
			       y, m, d, mjd3, u, a, rate);
			exit(0);
		}
		y += 1;
	}
}
