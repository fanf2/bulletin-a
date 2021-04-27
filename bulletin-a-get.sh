#!/bin/sh

set -eu

for i in $(jot -w %03d 16)
do
	url="https://datacenter.iers.org/data/6/bulletina-xxxiv-$i.txt"
	curl -o "bulletin-a-$i" "$url"
done
