#!/bin/bash
# Quick Git setup with placeholder values

git config user.name "Your Name"
git config user.email "your.email@example.com"

git add -A
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

git branch -M main

echo "Repository ready! Update git config with your real name/email:"
echo "git config user.name 'Your Actual Name'"
echo "git config user.email 'your.actual.email@example.com'"
