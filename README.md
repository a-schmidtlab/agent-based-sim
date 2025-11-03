# Predator-Prey Simulation

An educational Rust application that simulates an agent-based predator-prey ecosystem model with an interactive graphical user interface.

## Status

Project status: stable; suitable for educational use.

## Features

### Core Simulation
- **Agent-based predator-prey simulation** with realistic behaviors
- **Torus topography** (wraparound world) - agents seamlessly wrap around edges
- **Dynamic spawning** - spawn predators and prey during simulation
- **Real-time parameter adjustment** via sliders
- **Energy-based lifecycle** - agents consume energy, reproduce, and die naturally

### Interactive Controls
- **Population controls**: Adjust initial predator/prey counts with sliders (0–500)
- **Agent spawning**: Add predators or prey during execution via "Spawn Now" buttons
- **Reset**: Restart with current slider values
- **Pause/Resume**: Control execution state
- **Speed control**: Adjust simulation speed (0.1× to 5×)

### Live Evaluation & Graphics
- **Population graph**: Real-time line graph showing predator and prey population counts
  - Red line: predators
  - Green line: prey
  - Auto-scaling based on data range
- **Statistics window**:
  - Current population counts
  - Average energy levels
  - Historical aggregates (averages, peaks)
  - Data point tracking
- **Data collection**: Maintains a rolling window of the last 1000 ticks

### Visualization Options
- **Energy-based coloring**: Agent color encodes energy level
- **Velocity vectors**: Optional visualization of movement direction
- **Perception radius**: Optional visualization of detection ranges
- **Grid overlay**: Optional spatial reference grid
- **Adjustable agent size**: Configurable visualization scale

## Installation

```bash
cargo build --release
```

## Quick Start

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run --release
```

### Usage Guide

1. Start the application with default parameters
2. Adjust initial populations via the population sliders
3. Spawn agents during execution via the respective "Spawn Now" buttons
4. Observe population trends in the population graph window
5. Inspect current and historical metrics in the statistics window
6. Adjust parameters in the control panel sections:
   - Predator behavior (speed, perception, energy, reproduction)
   - Prey behavior (speed, detection, regeneration, reproduction)
   - World settings (size, boundary type)
   - Simulation settings (tick rate, max agents)

### Key Features Explained

**Torus topography**: The world wraps around at all edges. Agents crossing a boundary re-enter on the opposite edge. Distance calculations account for wraparound.

**Live spawning**: Agents can be added during execution without resetting. Typical uses include testing population responses and demonstrating intervention scenarios.

**Population graph**: Often exhibits predator–prey cycles. Increases in prey availability enable predator growth; elevated predation reduces prey, subsequently lowering predator counts.

## Theoretical Background: Agent-Based Simulation

Agent-based modeling (ABM) simulates systems from the bottom up. Individual entities (agents) follow local rules; aggregate behavior emerges from their interactions. In this system, agents are predators or prey with state (position, velocity, energy, age) and rule sets.

- **Agent rules**: Predators pursue prey within a perception radius, consume within a capture distance, lose energy over time, and reproduce above a threshold. Prey detect predators, flee, regenerate energy, and may reproduce.
- **Emergence**: Local rules can produce familiar ecological dynamics (e.g., predator–prey cycles). In contrast to equation-based mean-field models, ABM explicitly represents space, heterogeneity, and stochastic events.
- **Relation to Lotka–Volterra**: Lotka–Volterra provides coupled ODEs for average population dynamics. ABM introduces spatial structure and randomness, enabling clustering, local extinctions, dispersal effects, and run-to-run variability.
- **Topology**: A torus (wraparound) 2D space avoids edge artifacts; agents re-enter on the opposite edge. This approximates homogeneous conditions.
- **Time and stochasticity**: The simulation advances in discrete ticks. Random components (e.g., spawn positions) introduce variability and enable sensitivity analyses via repeated runs.

The objective is to provide a transparent reference implementation for teaching and experimentation.

## Documentation

See the `docs/` directory for comprehensive documentation:
- `USER_GUIDE.md` - Detailed usage instructions
- `ARCHITECTURE.md` - System architecture and design
- `SIMULATION_MODEL.md` - Mathematical model and algorithms
- `DEVELOPER_GUIDE.md` - Development and contribution guide
- `FEATURE_PLAN.md` - Implementation plan for new features

## Testing

Run all tests:
```bash
cargo test
```

Run specific test suites:
```bash
cargo test --test torus_tests      # Torus topography tests
cargo test --test spawn_tests     # Spawn functionality tests
cargo test --test integration_tests # Full simulation tests
```

## Technical Details

- **Language**: Rust (edition 2021)
- **GUI Framework**: egui/eframe (immediate mode)
- **Dependencies**: 
  - `egui`/`eframe` for GUI
  - `serde`/`toml` for configuration
  - `rand` for randomness
- **Performance**: Optimized for real-time simulation with 1000+ agents

2025 Axel Schmidt

