#!/bin/bash

# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 X Y"
    exit 1
fi

# Assign arguments to variables
x=$1
y=$2

# Validate that both arguments are integers
if ! [[ "$x" =~ ^[0-9]+$ ]] || ! [[ "$y" =~ ^[0-9]+$ ]]; then
    echo "Error: Both arguments must be integers."
    exit 1
fi

# Generate keypair files
for i in $(seq $x $y); do
    solana-keygen new -o "id${i}.json"
done
