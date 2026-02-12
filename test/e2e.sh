#! /bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Create random temporary directory for logs
LOG_DIR=$(mktemp -d)

# Check if spin is available
if ! command -v spin &> /dev/null; then
    echo -e "${RED}Error: Spin is not installed${NC}"
    echo "Please install Spin: https://developer.fermyon.com/spin/install"
    exit 1
fi
echo -e "${GREEN}✓ Spin is available${NC}"

# Build and install the plugin
echo -e "\n${GREEN}Building and installing the command trigger plugin...${NC}"
cargo build --release
spin pluginify --install

spin up --from examples/hello-world --quiet --log-dir "$LOG_DIR"

# Assert that the contents of stdout is `Hello, world!`
OUTPUT=$(cat "$LOG_DIR/hello-world_stdout.txt")
if [[ "$OUTPUT" == "Hello, world!" ]]; then
    echo -e "${GREEN}✓ Output is correct${NC}"
else
    echo -e "${RED}✗ Output is incorrect${NC}"
    echo "Expected: Hello, world!"
    echo "Got: $OUTPUT"
    exit 1
fi

rm -rf "$LOG_DIR"