#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
HOOKS_DIR="$PROJECT_DIR/.git/hooks"

echo "Installing git hooks..."

cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/bash

set -e

echo "Running cargo fmt..."
cargo fmt -- --check || {
    echo "❌ code is not formatted. Run 'cargo fmt' to fix."
    exit 1
} && echo "✅ code is formatted properly"

echo "Running cargo clippy..."
cargo clippy -- -D warnings || {
    echo "❌ clippy found issues. Fix them before committing."
    exit 1
} && echo "✅ clippy did not find any issues"

EOF

cat > "$HOOKS_DIR/pre-push" << 'EOF'
#!/bin/bash

set -e

echo "Running tests..."
cargo test -- --test-threads=2 || {
    echo "❌ Tests failed. Fix them before pushing."
    exit 1
}

echo "✅ All tests passed!"
EOF

chmod +x "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-push"

echo "✅ Git hooks installed successfully!"
echo ""
echo "Hooks installed:"
echo "  - pre-commit: runs cargo fmt and cargo clippy"
echo "  - pre-push: runs cargo test"
echo ""
echo "To skip hooks temporarily, use: git commit --no-verify"

