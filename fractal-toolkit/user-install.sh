#!/bin/bash

# Install Fractal Toolkit binaries to user's local bin directory

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_status "Installing Fractal Toolkit binaries..."

# Create ~/.local/bin if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binaries
cp target/release/ftk-mandel ~/.local/bin/
cp target/release/ftk-julia ~/.local/bin/
cp target/release/ftk-buddha ~/.local/bin/
cp target/release/ftk-buddhaj ~/.local/bin/

print_status "Binaries installed to ~/.local/bin/"
print_status "Make sure ~/.local/bin is in your PATH to use the commands."

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
    print_status "PATH already includes ~/.local/bin"
else
    print_warning "~/.local/bin is not in your PATH"
    print_warning "Add the following line to your ~/.bashrc or ~/.profile:"
    echo 'export PATH="$HOME/.local/bin:$PATH"'
fi

print_status "Installation completed!"
print_status "You can now use the Fractal Toolkit commands:"
print_status "  ftk-mandel --help    # Mandelbrot generator"
print_status "  ftk-julia --help     # Julia set generator"
print_status "  ftk-buddha --help    # Buddhabrot generator"
print_status "  ftk-buddhaj --help   # Buddhabrot Julia generator"