#!/bin/bash

# Directory containing crash files
CRASH_DIR="out/default/crashes/"
id
# Check if the directory exists
if [ ! -d "$CRASH_DIR" ]; then
  echo "Directory $CRASH_DIR does not exist."
  exit 1
fi

# Iterate over each file in the crash directory
for crash_file in "$CRASH_DIR"/id*; do
  if [ -f "$crash_file" ]; then
    echo "Processing $crash_file"
    cargo afl run --bin afl < "$crash_file"
  else
    echo "$crash_file is not a regular file, skipping."
  fi
done

echo "Done processing all crash files."
