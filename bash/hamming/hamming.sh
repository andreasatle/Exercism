#!/usr/bin/env bash

# Calculate the Hamming Distance between two DNA strands.
# 
# Your body is made up of cells that contain DNA. Those cells
# regularly wear out and need replacing, which they achieve by
# dividing into daughter cells. In fact, the average human body
# experiences about 10 quadrillion cell divisions in a lifetime!
# 
# When cells divide, their DNA replicates too. Sometimes during
# this process mistakes happen and single pieces of DNA get
# encoded with the incorrect information. If we compare two
# strands of DNA and count the differences between them we can
# see how many mistakes occurred. This is known as the
# "Hamming Distance".
# 
# We read DNA using the letters C,A,G and T. Two strands might
# look like this:
# 
# GAGCCTACTAACGGGAT
# CATCGTAATGACGGCCT
# ^ ^ ^  ^ ^    ^^
# They have 7 differences, and therefore the Hamming Distance is 7.

ERROR_MSG_EQUAL_LENGTH="left and right strands must be of equal length"
ERROR_MSG_USAGE="Usage: $0 <string1> <string2>"

hamming() {
    cnt=0
    for ((i=0; i < ${#1}; i++)) ; do
        [[ "${1:$i:1}" != "${2:$i:1}" ]] && ((cnt++))
    done
    echo $cnt
}

error() {
    [[ $# -eq 2 ]]                        \
        && echo ${ERROR_MSG_EQUAL_LENGTH} \
        || echo ${ERROR_MSG_USAGE}
}

[[ $# -eq 2 && ${#1} -eq ${#2} ]] \
    && hamming "$@"               \
    || error "$@"