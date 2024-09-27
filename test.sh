#!/bin/sh

cd tests/
clear
clang-format --style=file test.c > test_formatted.c
cargo r -rq test_formatted.c
cd ..