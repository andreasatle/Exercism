#!/usr/bin/env bash

# Your task is to convert a number into a string that contains
# raindrop sounds corresponding to certain potential factors.
# A factor is a number that evenly divides into another number,
# leaving no remainder. The simplest way to test if a one number
# is a factor of another is to use the modulo operation.
# 
# The rules of raindrops are that if a given number:
# 
# has 3 as a factor, add 'Pling' to the result.
# has 5 as a factor, add 'Plang' to the result.
# has 7 as a factor, add 'Plong' to the result.
# does not have any of 3, 5, or 7 as a factor, the result should be
# the digits of the number.

rain() {
    num=$1

    # Add a Pling if 3|num
    [[ `expr $num % 3` -eq 0 ]] \
        && str="Pling"

    # Add a Plang if 5|num
    [[ `expr $num % 5` -eq 0 ]] \
        && str="${str:-}Plang"

    # Add a Plong if 7|num
    [[ `expr $num % 7` -eq 0 ]] \
        && str="${str:-}Plong"

    # Return the string if non-empty, else return the arg
    echo ${str:-$num}
}

# Call function rain if input parameters are valid.
[[ $# -ge 1 && $1 =~ ^[0-9]+$ ]]     \
    && echo $(rain $1)               \
    || echo "Something went wrong!"
