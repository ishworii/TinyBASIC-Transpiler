#!/bin/bash

# Define the path to your Rust project and the GCC compiler
RUST_PROGRAM="/home/ik-pc/Desktop/rust_practice/tiny_compiler"
CC="gcc"


# Function to compile the Rust project and then compile the generated C file
function compile_and_run {
    # Navigate to the Rust project directory
    cd "$RUST_PROJECT_PATH" || { echo "Failed to enter directory $RUST_PROJECT_PATH"; exit 1; }

    # Compile the Rust project with Cargo
    echo "Compiling Rust project..."
    cargo build || { echo "Rust compilation failed"; exit 1; }

    # Assuming the Rust program generates a file named 'out.c' in the current directory
    GENERATED_C_FILE="out.c"
    OUTPUT_EXECUTABLE="output"

    # Check if the generated C file exists
    if [ -f "$GENERATED_C_FILE" ]; then
        # Compile the generated C file
        echo "Compiling generated C file..."
        $CC -o output "$GENERATED_C_FILE" || { echo "C compilation failed"; exit 1; }
        echo "Compilation successful. Executable created: output"

         # Run the compiled executable
        echo "Running the compiled executable..."
        ./$OUTPUT_EXECUTABLE
    else
        echo "Generated C file not found"
        exit 1
    fi
}

# Call the function
compile_and_run

