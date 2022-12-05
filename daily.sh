#!/bin/bash

# Check if a directory name was provided as the first argument
if [ -z "$1" ]; then
  echo "Please provide a directory name as the first argument."
  exit 1
fi

# Create the directory with the provided name
mkdir "$1"
touch "$1/example.txt"
touch "$1/input.txt"
touch "$1/solution.py"