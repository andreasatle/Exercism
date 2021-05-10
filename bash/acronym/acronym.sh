#!/usr/bin/env bash

# Convert a phrase to its acronym.
#
# Techies love their TLA (Three Letter Acronyms)!
#
# Help generate some jargon by writing a program that converts
# a long name like Portable Network Graphics to its acronym (PNG).

process_word() {
    tla=""
    for ((i=0; i<${#1}; i++)) ; do
        [[ ${1:i:1} =~ [A-Za-z] ]] && { tla=${1:i:1}; break; }
    done
    for ((i=1; i<${#1}; i++)) ; do
        [[ ${1:i:1} =~ [A-Z] && ${1:i-1:1} =~ [a-z] ]] && tla+=${1:i:1}
    done
    echo ${tla} # upper case
}

input_without_dashes="${@//["-"]/" "}"

IFS=' '
read -ra words <<< "${input_without_dashes}"

acro=""
for word in "${words[@]}" ; do
    acro+="$(process_word "$word")"
done
echo ${acro^^}