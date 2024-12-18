#!/bin/bash

# Copy the template file to the specified directory
cp -r template $1

# replace aoc-([0-9]{4})-XX with aoc-([0-9]{4})-$1 in Cargo.toml

sed -Ei "s/aoc-([0-9]{4})-XX/aoc-\1-$1/g" $1/Cargo.toml



