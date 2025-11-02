# Predator-Prey Simulation

An educational Rust application that simulates an agent-based predator-prey ecosystem model with an interactive graphical user interface.

## Status

✅ Fully functional - Ready for educational use!

## Features

### Core Simulation
- **Agent-based predator-prey simulation** with realistic behaviors
- **Torus topography** (wraparound world) - agents seamlessly wrap around edges
- **Dynamic spawning** - spawn predators and prey during simulation
- **Real-time parameter adjustment** via sliders
- **Energy-based lifecycle** - agents consume energy, reproduce, and die naturally

### Interactive Controls
- **Population Controls**: Adjust initial predator/prey counts with sliders (0-500 range)
- **Spawn Agents**: Add predators or prey to running simulation with "Spawn Now" buttons
- **Reset Simulation**: Restart with current slider values
- **Pause/Resume**: Control simulation execution
- **Speed Control**: Adjust simulation speed (0.1x to 5x)

### Live Evaluation & Graphics
- **Population Graph**: Real-time line graph showing predator and prey populations over time
  - Red line: Predator population
  - Green line: Prey population
  - Auto-scaling based on data range
- **Statistics Window**: 
  - Current population counts
  - Average energy levels
  - Historical statistics (averages, peaks)
  - Data point tracking
- **Data Collection**: Tracks last 1000 simulation ticks

### Visualization Options
- **Energy-based coloring**: Agent colors indicate energy levels
- **Velocity vectors**: Optional display of agent movement direction
- **Perception radius**: Visualize detection ranges
- **Grid overlay**: Optional grid for spatial reference
- **Adjustable agent size**: Customize visualization scale

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

1. **Start the simulation**: The app opens with default parameters
2. **Adjust populations**: Use the "Population Controls" sliders to set initial counts
3. **Spawn agents**: Click "🐺 Spawn Now" or "🐰 Spawn Now" to add agents during simulation
4. **Monitor populations**: Watch the Population Graph window for real-time trends
5. **View statistics**: Check the Statistics window for detailed metrics
6. **Adjust parameters**: Use collapsible sections in the control panel to modify:
   - Predator behavior (speed, perception, energy, reproduction)
   - Prey behavior (speed, detection, regeneration, reproduction)
   - World settings (size, boundary type)
   - Simulation settings (tick rate, max agents)

### Key Features Explained

**Torus Topography**: The world wraps around at all edges. Agents moving off one side appear on the opposite side. Distance calculations account for this, so predators can detect prey across boundaries.

**Live Spawning**: You can add agents to a running simulation without resetting. Useful for:
- Testing population dynamics
- Observing how ecosystems respond to changes
- Educational demonstrations

**Population Graph**: Shows the classic predator-prey cycle:
- When prey population rises, predators have more food and multiply
- High predator population reduces prey numbers
- Reduced prey causes predator decline
- The cycle repeats

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

## License

*(To be determined)*

