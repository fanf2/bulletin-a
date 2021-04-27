#!/bin/sh

set -eu

for f in bulletin-a-0*
do
	./bulletin-a.pl $f
done
