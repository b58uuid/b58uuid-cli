#!/bin/sh
# B58UUID CLI Installation Script
# Usage: curl -fsSL https://b58uuid.io/install.sh | sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="b58uuid/b58uuid-cli"
BINARY_NAME="b58uuid"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="darwin"
            ;;
        *)
            echo "${RED}Error: Unsupported operating system: $OS${NC}"
            exit 1
            ;;
    esac
    
    case "$ARCH" in
        x86_64|amd64)
            ARCH="amd64"
            ;;
        aarch64|arm64)
            ARCH="arm64"
            ;;
        *)
            echo "${RED}Error: Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac
    
    PLATFORM="${OS}-${ARCH}"
}

# Get latest release version
get_latest_version() {
    echo "${YELLOW}Fetching latest version...${NC}"
    VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        echo "${RED}Error: Failed to fetch latest version${NC}"
        exit 1
    fi
    
    echo "${GREEN}Latest version: ${VERSION}${NC}"
}

# Download and install
install_binary() {
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}-${PLATFORM}.tar.gz"
    TMP_DIR=$(mktemp -d)
    
    echo "${YELLOW}Downloading ${BINARY_NAME} ${VERSION} for ${PLATFORM}...${NC}"
    
    if ! curl -fsSL "$DOWNLOAD_URL" -o "${TMP_DIR}/${BINARY_NAME}.tar.gz"; then
        echo "${RED}Error: Failed to download ${DOWNLOAD_URL}${NC}"
        rm -rf "$TMP_DIR"
        exit 1
    fi
    
    echo "${YELLOW}Extracting...${NC}"
    tar -xzf "${TMP_DIR}/${BINARY_NAME}.tar.gz" -C "$TMP_DIR"
    
    echo "${YELLOW}Installing to ${INSTALL_DIR}...${NC}"
    
    # Try to install with sudo if needed
    if [ -w "$INSTALL_DIR" ]; then
        mv "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
        chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    else
        echo "${YELLOW}Requesting sudo access to install to ${INSTALL_DIR}...${NC}"
        sudo mv "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
        sudo chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    fi
    
    rm -rf "$TMP_DIR"
    
    echo "${GREEN}✓ ${BINARY_NAME} installed successfully!${NC}"
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        VERSION_OUTPUT=$("$BINARY_NAME" --version)
        echo "${GREEN}✓ Installation verified: ${VERSION_OUTPUT}${NC}"
    else
        echo "${YELLOW}Warning: ${BINARY_NAME} is installed but not in PATH${NC}"
        echo "Add ${INSTALL_DIR} to your PATH or run: export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

# Main
main() {
    echo "${GREEN}B58UUID CLI Installer${NC}"
    echo ""
    
    detect_platform
    get_latest_version
    install_binary
    verify_installation
    
    echo ""
    echo "${GREEN}Installation complete!${NC}"
    echo ""
    echo "Try it out:"
    echo "  ${BINARY_NAME} encode 550e8400-e29b-41d4-a716-446655440000"
    echo "  ${BINARY_NAME} generate"
    echo "  ${BINARY_NAME} --help"
    echo ""
    echo "For more information, visit: https://b58uuid.io"
}

main
