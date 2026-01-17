#!/bin/bash
# Deploy RizzScript documentation to gh-pages branch

set -e

echo "🚀 Deploying RizzScript documentation to gh-pages..."

# Check if we're in the right directory
if [ ! -d "docs" ]; then
  echo "❌ Error: docs directory not found. Run this script from the repository root."
  exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
  echo "❌ Error: Node.js is not installed. Please install Node.js first."
  exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
  echo "❌ Error: npm is not installed. Please install npm first."
  exit 1
fi

# Navigate to docs directory
cd docs

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Build documentation
echo "🔨 Building documentation..."
npm run build

# Check if build was successful
if [ ! -d ".vitepress/dist" ]; then
  echo "❌ Error: Build failed. .vitepress/dist directory not found."
  exit 1
fi

# Go back to repo root
cd ..

# Check if git is installed
if ! command -v git &> /dev/null; then
  echo "❌ Error: git is not installed. Please install git first."
  exit 1
fi

# Check if we're in a git repository
if [ ! -d ".git" ]; then
  echo "❌ Error: Not a git repository. Please run this script from the repository root."
  exit 1
fi

# Stash any uncommitted changes
echo "💾 Stashing uncommitted changes..."
git stash

# Switch to gh-pages branch or create it
echo "🌿 Switching to gh-pages branch..."
if git show-ref --verify --quiet refs/heads/gh-pages; then
  git checkout gh-pages
  git pull origin gh-pages || true
else
  git checkout --orphan gh-pages
  git rm -rf --cached . || true
fi

# Remove all files except .git
echo "🧹 Cleaning gh-pages branch..."
find . -maxdepth 1 ! -name '.' ! -name '.git' ! -name 'docs' -exec rm -rf {} + || true

# Copy built documentation
echo "📋 Copying built documentation..."
cp -r docs/.vitepress/dist/* .

# Add all files
git add .

# Commit changes
echo "💬 Committing changes..."
git commit -m "Deploy documentation $(date '+%Y-%m-%d %H:%M:%S')" || {
  echo "⚠️  No changes to commit."
}

# Push to gh-pages branch
echo "🚀 Pushing to gh-pages branch..."
git push origin gh-pages || {
  echo "❌ Error: Failed to push to gh-pages branch."
  echo "💡 Make sure you have push access to the repository."
  exit 1
}

# Switch back to original branch
echo "🔄 Switching back to original branch..."
git checkout main || git checkout master || {
  echo "⚠️  Could not switch back to main/master branch."
}

# Restore stashed changes
echo "📦 Restoring stashed changes..."
git stash pop || true

echo "✅ Documentation deployed successfully!"
echo "🌐 Your documentation should be available at: https://rizz-script.github.io/rizz/"
