#!/usr/bin/env bash

# Two-fer or 2-fer is short for two for one.
# One for you and one for me.
# 
# Given a name, return a string with the message:
# 
# One for name, one for me.
# Where "name" is the given name.
# 
# If the name is missing, return the string:
# 
# One for you, one for me.

echo "One for "${1:-you}", one for me."

