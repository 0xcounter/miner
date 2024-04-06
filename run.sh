# Set x to the first argument, default to an empty string if not provided
x=${1:-}

# Loop and run the cargo command using the constructed filename
for i in {1..10000}; do
    cargo run -- --keypair "id${x}.json" --priority-fee 100000 mine
done