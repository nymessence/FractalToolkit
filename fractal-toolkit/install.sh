#!/bin/bash

# Fractal Toolkit Installation Script
# This script compiles and installs the Fractal Toolkit binaries to the system

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on CI or with non-interactive shell
if [[ -z "${CI}" && -t 0 ]]; then
    INTERACTIVE=1
else
    INTERACTIVE=0
fi

# Default installation directory
DEFAULT_PREFIX="/usr/local"
PREFIX="${INSTALL_PREFIX:-$DEFAULT_PREFIX}"

# Help function
show_help() {
    cat << EOF
Fractal Toolkit Installation Script

This script compiles and installs the Fractal Toolkit binaries.

Usage: $0 [OPTIONS]

Options:
    -h, --help          Show this help message
    --prefix=PATH       Installation prefix (default: $DEFAULT_PREFIX)
    --release           Build in release mode (default)
    --debug             Build in debug mode
    --skip-build        Skip the build process (use existing binaries)
    --uninstall         Uninstall the toolkit

Examples:
    $0                              # Install with default settings
    $0 --prefix=/opt                # Install to /opt
    $0 --debug                      # Build debug version and install
    $0 --skip-build                 # Install existing binaries without rebuilding
EOF
}

# Parse command line arguments
BUILD_MODE="release"
SKIP_BUILD=0
UNINSTALL=0

for arg in "$@"; do
    case $arg in
        -h|--help)
            show_help
            exit 0
            ;;
        --prefix=*)
            PREFIX="${arg#*=}"
            ;;
        --release)
            BUILD_MODE="release"
            ;;
        --debug)
            BUILD_MODE="debug"
            ;;
        --skip-build)
            SKIP_BUILD=1
            ;;
        --uninstall)
            UNINSTALL=1
            ;;
        *)
            print_error "Unknown option: $arg"
            show_help
            exit 1
            ;;
    esac
done

# Check if Rust and Cargo are installed
check_dependencies() {
    print_status "Checking dependencies..."

    if ! command -v rustc &> /dev/null; then
        print_error "Rust compiler (rustc) is not installed or not in PATH"
        print_warning "Please install Rust from https://rustup.rs/"
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        print_error "Cargo (Rust package manager) is not installed or not in PATH"
        print_warning "Please install Rust from https://rustup.rs/"
        exit 1
    fi

    print_status "Rust and Cargo are available"
    print_status "Rust version: $(rustc --version)"
    print_status "Cargo version: $(cargo --version)"
}

# Build the project
build_project() {
    if [ $SKIP_BUILD -eq 1 ]; then
        print_status "Skipping build (--skip-build flag set)"
        return 0
    fi

    print_status "Building Fractal Toolkit in $BUILD_MODE mode..."
    
    if [ "$BUILD_MODE" = "release" ]; then
        cargo build --release
        print_status "Release build completed successfully"
    else
        cargo build
        print_status "Debug build completed successfully"
    fi
}

# Install the binaries
install_binaries() {
    print_status "Installing binaries to $PREFIX/bin..."

    # Create bin directory if it doesn't exist
    sudo mkdir -p "$PREFIX/bin"

    # Determine the source directory based on build mode
    if [ "$BUILD_MODE" = "release" ]; then
        SOURCE_DIR="target/release"
    else
        SOURCE_DIR="target/debug"
    fi

    # Check if the source directory exists and has binaries
    if [ ! -d "$SOURCE_DIR" ]; then
        print_error "Build directory $SOURCE_DIR does not exist"
        print_warning "Run the build step first or use --skip-build with existing binaries"
        exit 1
    fi

    # Find and install all fractal toolkit binaries
    for binary in ftk-mandel ftk-julia ftk-buddha ftk-buddhaj; do
        if [ -f "$SOURCE_DIR/$binary" ]; then
            print_status "Installing $binary..."
            sudo cp "$SOURCE_DIR/$binary" "$PREFIX/bin/"
            sudo chmod +x "$PREFIX/bin/$binary"
        else
            print_error "$binary not found in $SOURCE_DIR"
            print_warning "Make sure the project was built successfully"
            exit 1
        fi
    done

    print_status "All binaries installed successfully to $PREFIX/bin/"
}

# Uninstall the binaries
uninstall_binaries() {
    print_status "Uninstalling Fractal Toolkit from $PREFIX/bin..."

    for binary in ftk-mandel ftk-julia ftk-buddha ftk-buddhaj; do
        if [ -f "$PREFIX/bin/$binary" ]; then
            print_status "Removing $binary..."
            sudo rm "$PREFIX/bin/$binary"
        else
            print_warning "$binary not found in $PREFIX/bin (already uninstalled?)"
        fi
    done

    print_status "Uninstall completed"
}

# Verify installation
verify_installation() {
    print_status "Verifying installation..."

    for binary in ftk-mandel ftk-julia ftk-buddha ftk-buddhaj; do
        if command -v "$binary" &> /dev/null; then
            print_status "$binary is correctly installed and in PATH"
        else
            print_warning "$binary is not in PATH - you may need to add $PREFIX/bin to your PATH"
        fi
    done

    print_status "Installation verification completed"
    print_status "You can now use the Fractal Toolkit commands:"
    print_status "  ftk-mandel --help    # Mandelbrot generator"
    print_status "  ftk-julia --help     # Julia set generator" 
    print_status "  ftk-buddha --help    # Buddhabrot generator"
    print_status "  ftk-buddhaj --help   # Buddhabrot Julia generator"
}

# Main execution
main() {
    print_status "Starting Fractal Toolkit installation"
    print_status "Installation prefix: $PREFIX"
    print_status "Build mode: $BUILD_MODE"

    if [ $UNINSTALL -eq 1 ]; then
        uninstall_binaries
        exit 0
    fi

    check_dependencies
    build_project
    install_binaries
    verify_installation

    print_status "Installation completed successfully!"
    print_status "The Fractal Toolkit is now ready to use."
}

# Run main function
main "$@"