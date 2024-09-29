#!/bin/sh

cd tests/
clear
clang-format --style=file test.c > test_formatted.c
clang-format --style=file test.h > test_formatted.h
cargo r -rq test_formatted.c test_formatted.h
cd ..