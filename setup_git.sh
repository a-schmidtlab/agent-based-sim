#!/bin/bash
# Git setup script for predator-prey-sim

echo "Setting up Git repository for GitHub..."

# Set local git config (repository only, not global)
read -p "Enter your name for Git commits: " git_name
read -p "Enter your email for Git commits: " git_email

git config user.name "$git_name"
git config user.email "$git_email"

echo "✓ Git user configured"
echo ""

# Stage all files
echo "Staging all files..."
git add -A
echo "✓ Files staged"
echo ""

# Create initial commit
echo "Creating initial commit..."
git commit -m "Initial commit: Predator-Prey Simulation with GUI

Features:
- Agent-based predator-prey simulation
- Torus topography (wraparound world)
- Interactive GUI with egui/eframe
- Real-time parameter adjustment via sliders
- Dynamic agent spawning
- Live population graphs and statistics
- Comprehensive test suite (76 tests)
- Full documentation

Ready for educational use!"

echo "✓ Initial commit created"
echo ""

# Rename branch to main
git branch -M main
echo "✓ Branch renamed to 'main'"
echo ""

echo "========================================="
echo "Repository is ready for GitHub!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. Create a new repository on GitHub:"
echo "   https://github.com/new"
echo ""
echo "2. After creating the repo, run:"
echo "   git remote add origin https://github.com/YOUR_USERNAME/predator-prey-sim.git"
echo "   git push -u origin main"
echo ""
echo "Or see GITHUB_SETUP.md for detailed instructions."

