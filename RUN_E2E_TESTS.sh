#!/bin/bash

# General Bots E2E Testing Script
# Run end-to-end tests: Platform Load → Login → Chat → Logout
# Usage: ./RUN_E2E_TESTS.sh [option]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  General Bots E2E Testing Framework                           ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_section() {
    echo -e "${YELLOW}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

show_help() {
    print_header
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Options:"
    echo "  1, http        Run HTTP-only test (no browser needed, ~2 seconds)"
    echo "  2, startup     Run BotServer startup test (~5 seconds)"
    echo "  3, complete    Run complete platform flow with browser (~45 seconds)"
    echo "  4, headless    Run complete flow in headless mode (default)"
    echo "  5, headed      Run complete flow with visible browser (for debugging)"
    echo "  6, clean       Clean build artifacts"
    echo "  7, setup-wd    Setup WebDriver (chromedriver)"
    echo "  help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 http              # Quick test without browser"
    echo "  $0 complete          # Full E2E test with browser"
    echo "  $0 headed            # See browser in action (debugging)"
    echo ""
}

check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not installed"
        echo "Install from: https://rustup.rs/"
        exit 1
    fi
    print_success "Rust toolchain found"
}

check_webdriver() {
    if ! command -v chromedriver &> /dev/null; then
        print_error "chromedriver not found in PATH"
        echo ""
        print_info "WebDriver options:"
        echo "  1. Download from: https://chromedriver.chromium.org/"
        echo "  2. Or run with Docker: docker run -d -p 4444:4444 selenium/standalone-chrome"
        echo "  3. Or run: $0 setup-wd"
        return 1
    fi
    print_success "chromedriver found"
    return 0
}

setup_webdriver() {
    print_section "Setting up WebDriver..."

    if command -v docker &> /dev/null; then
        print_info "Starting Selenium WebDriver in Docker..."
        docker run -d -p 4444:4444 selenium/standalone-chrome || true
        print_success "WebDriver started on port 4444"
    else
        print_error "Docker not found. Install Docker or chromedriver manually."
        echo ""
        echo "Manual setup:"
        echo "  1. Download chromedriver: https://chromedriver.chromium.org/"
        echo "  2. Place in /usr/local/bin/"
        echo "  3. Run: chromedriver --port=4444"
        exit 1
    fi
}

run_http_only_test() {
    print_section "Running HTTP-Only Platform Loading Test"
    echo "Tests: Platform health check, API endpoints, database"
    echo ""

    cd "$(dirname "$0")"
    cargo test --test e2e --features e2e test_platform_loading_http_only -- --nocapture

    print_success "HTTP-Only test completed"
}

run_startup_test() {
    print_section "Running BotServer Startup Test"
    echo "Tests: Server process, configuration, dependencies"
    echo ""

    cd "$(dirname "$0")"
    cargo test --test e2e --features e2e test_botserver_startup -- --nocapture

    print_success "Startup test completed"
}

run_complete_test_headless() {
    print_section "Running Complete Platform Flow (Headless)"
    echo "Tests: Load → Login → Chat → Logout"
    echo ""

    if ! check_webdriver; then
        print_error "WebDriver required for complete test"
        echo ""
        print_info "Start WebDriver:"
        echo "  docker run -d -p 4444:4444 selenium/standalone-chrome"
        echo ""
        echo "Or use HTTP-only test: $0 http"
        exit 1
    fi

    cd "$(dirname "$0")"
    cargo test --test e2e --features e2e test_complete_platform_flow_login_chat_logout -- --nocapture

    print_success "Complete platform flow test passed"
}

run_complete_test_headed() {
    print_section "Running Complete Platform Flow (With Browser UI)"
    echo "Tests: Load → Login → Chat → Logout (VISIBLE)"
    echo ""

    if ! check_webdriver; then
        print_error "WebDriver required for complete test"
        exit 1
    fi

    print_info "Watch the browser window as the test runs..."
    echo ""

    cd "$(dirname "$0")"
    HEADED=1 RUST_LOG=debug cargo test --test e2e --features e2e test_complete_platform_flow_login_chat_logout -- --nocapture --test-threads=1

    print_success "Complete platform flow test passed"
}

clean_build() {
    print_section "Cleaning build artifacts"
    cd "$(dirname "$0")"
    cargo clean
    print_success "Build artifacts cleaned"
}

# Main execution
print_header

OPTION="${1:-help}"

case "$OPTION" in
    1|http)
        check_rust
        run_http_only_test
        ;;
    2|startup)
        check_rust
        run_startup_test
        ;;
    3|complete|headless)
        check_rust
        run_complete_test_headless
        ;;
    4|headed)
        check_rust
        run_complete_test_headed
        ;;
    5|clean)
        clean_build
        ;;
    6|setup-wd)
        setup_webdriver
        ;;
    help|-h|--help)
        show_help
        ;;
    *)
        print_error "Unknown option: $OPTION"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

echo ""
print_success "All done!"
echo ""
