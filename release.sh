#!/bin/bash
set -e

# Ensure we're on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "Error: Must be on main branch (currently on $BRANCH)"
    exit 1
fi

# Build frontend
echo "Building frontend..."
npm run build

# Check if dist changed
if git diff --quiet dist/; then
    echo "Frontend bundle is up to date"
else
    echo "Committing updated frontend bundle..."
    git add dist/
    git commit -m "build: update frontend bundle"
fi

# Get current version
CURRENT_VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
echo "Current version: $CURRENT_VERSION"

# Ask for new version
read -p "New version (or press Enter to skip version bump): " NEW_VERSION

if [ -n "$NEW_VERSION" ]; then
    # Update Cargo.toml
    sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

    # Commit version bump
    git add Cargo.toml
    git commit -m "v$NEW_VERSION Release"

    # Tag
    git tag "v$NEW_VERSION"

    echo "Tagged v$NEW_VERSION"
    echo ""
    echo "To publish:"
    echo "  git push origin main --tags"
    echo "  cargo publish"
else
    echo "Skipping version bump"
fi
