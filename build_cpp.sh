#!/usr/bin/env bash
# author:	Jacob Xie
# date:	2024/07/29 19:39:00 Monday
# brief:

# Check if exactly one argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <cpp_filename>"
    exit 1
fi

# Extract the argument
cpp_fname=$1

# Extract the base name of the file (without extension)
base_name=$(basename "$cpp_fname" .cpp)

# Compile the C++ file using g++
g++ -std=c++20 -o "$base_name" "$cpp_fname"

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "Compilation successful. Executable is ./$base_name"

    mkdir -p bin

    mv "$base_name" bin/
    echo "Executable moved to ./bin/$base_name"
else
    echo "Compilation failed."
    exit 1
fi
