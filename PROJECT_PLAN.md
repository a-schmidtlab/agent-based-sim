# Predator-Prey Simulation Project Plan

## Project Overview
An educational Rust application that simulates an agent-based predator-prey ecosystem model with an interactive graphical user interface, real-time parameter adjustment via sliders, and comprehensive documentation.

## Goals
- Educational tool for teaching ecology, agent-based modeling, and simulation concepts
- Interactive visualization of predator-prey dynamics
- Real-time parameter manipulation for experimentation
- Well-documented codebase for learning and extension

---

## Technology Stack

### Core Language
- **Rust** - Performance, safety, and modern ecosystem

### Graphics & UI Framework
- **egui** or **iced** - Immediate mode GUI framework for Rust
  - egui: Simpler, immediate mode, great for sliders and controls
  - iced: More modern, elm-like architecture
  - **Recommendation: egui** - Easier to integrate, better for real-time controls

### Rendering
- **minifb** or **pixels** or **wgpu** (via egui)
  - If using egui: Use egui's built-in rendering
  - Standalone option: **pixels** for pixel-perfect control
  - **Recommendation: egui + custom canvas** - Simplest for 2D visualization

### Mathematics & Simulation
- Standard Rust (no external math library needed for basic simulation)
- Optional: **nalgebra** for complex vector operations if needed

### Configuration
- **serde** + **toml** or **ron** - For saving/loading parameter presets

---

## Project Structure

```
predator-prey-sim/
├── Cargo.toml
├── README.md
├── docs/
│   ├── ARCHITECTURE.md
│   ├── SIMULATION_MODEL.md
│   ├── USER_GUIDE.md
│   ├── DEVELOPER_GUIDE.md
│   └── EXAMPLES.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── simulation/
│   │   ├── mod.rs
│   │   ├── agent.rs          # Base agent trait/struct
│   │   ├── predator.rs       # Predator agent implementation
│   │   ├── prey.rs           # Prey agent implementation
│   │   ├── world.rs          # World/environment management
│   │   └── rules.rs          # Simulation rules and logic
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── controls.rs       # Sliders, buttons, parameter UI
│   │   ├── visualization.rs  # Canvas rendering of agents
│   │   └── layout.rs         # UI layout management
│   ├── config/
│   │   ├── mod.rs
│   │   └── parameters.rs     # Simulation parameters struct
│   └── utils/
│       ├── mod.rs
│       ├── math.rs           # Vector math, distance calculations
│       └── color.rs          # Color schemes for visualization
├── examples/
│   ├── basic_simulation.rs
│   └── custom_rules.rs
├── tests/
│   ├── simulation_tests.rs
│   └── agent_tests.rs
└── assets/
    └── (optional icons, presets)
```

---

## Core Components

### 1. Agent System (`simulation/agent.rs`)
**Purpose**: Base abstraction for all agents in the simulation

**Features**:
- Position (x, y coordinates)
- Velocity vector
- Energy/health level
- Age/lifetime tracking
- Unique ID
- Base behavior traits (move, perceive, act)

**Design**:
```rust
pub trait Agent {
    fn update(&mut self, world: &World, dt: f64) -> AgentAction;
    fn position(&self) -> Vector2;
    fn energy(&self) -> f64;
    fn is_alive(&self) -> bool;
}

pub struct BaseAgent {
    id: u32,
    position: Vector2,
    velocity: Vector2,
    energy: f64,
    age: u32,
    // ...
}
```

### 2. Predator Agent (`simulation/predator.rs`)
**Purpose**: Implements predator-specific behaviors

**Features**:
- Chase nearby prey
- Consume prey for energy gain
- Starvation when energy depletes
- Reproduction when energy threshold reached
- Perception radius for detecting prey
- Maximum speed
- Energy consumption rate

**Parameters**:
- Initial energy
- Max speed
- Perception radius
- Energy gain from eating
- Energy consumption per tick
- Reproduction threshold
- Reproduction cost

### 3. Prey Agent (`simulation/prey.rs`)
**Purpose**: Implements prey-specific behaviors

**Features**:
- Flee from nearby predators
- Forage for food (regenerating energy)
- Reproduction when conditions met
- Natural energy regeneration
- Escape behavior (flee in opposite direction)

**Parameters**:
- Initial energy
- Max speed
- Detection radius for predators
- Energy regeneration rate
- Reproduction threshold
- Energy cost for reproduction

### 4. World/Environment (`simulation/world.rs`)
**Purpose**: Manages the simulation environment and all agents

**Features**:
- Bounded or toroidal world (wraparound edges)
- Agent storage and management
- Spatial partitioning (grid or quadtree) for efficient collision detection
- Food sources (optional - stationary energy sources)
- Boundary conditions
- Tick/simulation step management

**Responsibilities**:
- Update all agents
- Handle collisions and interactions
- Manage agent lifecycle (spawn, death, removal)
- Provide spatial queries (find nearby agents)

### 5. Simulation Rules (`simulation/rules.rs`)
**Purpose**: Implements the core simulation logic

**Features**:
- Agent interaction rules (predator eats prey, prey flees)
- Distance-based detection
- Energy transfer
- Reproduction mechanics
- Death conditions
- Movement physics

### 6. Parameters (`config/parameters.rs`)
**Purpose**: Centralized configuration for all simulation parameters

**Features**:
- Predator parameters (speed, energy, perception, etc.)
- Prey parameters (speed, energy, detection, etc.)
- World parameters (size, boundaries, food spawn rate)
- Simulation parameters (tick rate, max agents, etc.)
- Serialization support for saving/loading presets

### 7. UI Controls (`ui/controls.rs`)
**Purpose**: Interactive parameter adjustment

**Features**:
- Sliders for all parameters
  - Predator: speed, perception, energy settings
  - Prey: speed, detection, regeneration
  - World: size, food spawn
  - Simulation: tick rate, pause/resume
- Buttons:
  - Start/Stop/Pause
  - Reset simulation
  - Save/Load presets
  - Clear all agents
  - Spawn agents (predators/prey)
- Real-time value display
- Parameter grouping (collapsible sections)

### 8. Visualization (`ui/visualization.rs`)
**Purpose**: Render the simulation visually

**Features**:
- Canvas/window rendering
- Agent visualization:
  - Different colors/shapes for predators vs prey
  - Size based on energy level
  - Direction indicator (velocity vector)
- Grid overlay (optional)
- Statistics overlay:
  - Current population counts
  - Average energy levels
  - Generation count
  - Simulation time/tick count
- Performance metrics (FPS, update time)

### 9. Utilities
**Math utilities**: Vector operations, distance calculations, angle calculations
**Color utilities**: Color schemes, gradients for energy visualization

---

## Features & Functionality

### Phase 1: Core Simulation (MVP)
- [ ] Basic agent system (base agent trait)
- [ ] Predator agent implementation
- [ ] Prey agent implementation
- [ ] Simple movement physics
- [ ] Basic interaction (predator eats prey)
- [ ] Energy system
- [ ] Death when energy reaches zero
- [ ] World boundaries (wraparound or walls)

### Phase 2: Advanced Simulation
- [ ] Reproduction mechanics
- [ ] Perception/detection system
- [ ] Flee/chase behaviors
- [ ] Spatial partitioning for performance
- [ ] Food sources (optional energy for prey)
- [ ] Age/lifetime tracking
- [ ] Mutation/variation (optional - for educational value)

### Phase 3: UI & Visualization
- [ ] Basic window setup with egui
- [ ] Canvas rendering of agents
- [ ] Parameter sliders (all key parameters)
- [ ] Control buttons (start/stop/reset)
- [ ] Real-time statistics display
- [ ] Zoom and pan (optional)

### Phase 4: Polish & Features
- [ ] Save/Load parameter presets
- [ ] Export statistics to CSV
- [ ] Multiple visualization modes
- [ ] Performance optimization
- [ ] Keyboard shortcuts
- [ ] Help/instructions overlay

### Phase 5: Documentation
- [ ] Comprehensive README
- [ ] Architecture documentation
- [ ] Simulation model explanation
- [ ] User guide with screenshots
- [ ] Developer guide
- [ ] Inline code documentation
- [ ] Example scenarios
- [ ] Educational use cases

---

## Simulation Model Details

### Agent States
1. **Alive**: Normal operation, can move, interact, reproduce
2. **Dead**: Removed from simulation, no interactions

### Movement Model
- Agents move based on velocity vectors
- Velocity updated based on:
  - Predator: Direction toward nearest prey
  - Prey: Direction away from nearest predator
  - Random component (optional - for exploration)
- Speed limited by max_speed parameter

### Interaction Model
- **Predator-Prey**:
  - Predator detects prey within perception_radius
  - Predator moves toward prey
  - When distance < capture_distance: prey consumed
  - Predator gains energy, prey dies
  
- **Reproduction**:
  - When energy > reproduction_threshold
  - Spawn new agent nearby
  - Parent loses reproduction_cost energy
  - Offspring inherits parameters (with optional mutation)

### Energy Model
- Predator: Loses energy per tick, gains from eating
- Prey: Natural regeneration, may lose when fleeing

---

## Parameter Categories

### Predator Parameters
- `initial_energy`: Starting energy level
- `max_speed`: Maximum movement speed
- `perception_radius`: Distance to detect prey
- `capture_distance`: Distance to consume prey
- `energy_per_tick`: Energy lost each simulation step
- `energy_gain_from_prey`: Energy gained when eating
- `reproduction_threshold`: Energy needed to reproduce
- `reproduction_cost`: Energy lost when reproducing
- `initial_count`: Starting number of predators

### Prey Parameters
- `initial_energy`: Starting energy level
- `max_speed`: Maximum movement speed
- `detection_radius`: Distance to detect predators
- `flee_distance`: Distance to trigger fleeing
- `energy_regeneration`: Energy gained per tick
- `energy_loss_fleeing`: Extra energy lost when fleeing
- `reproduction_threshold`: Energy needed to reproduce
- `reproduction_cost`: Energy lost when reproducing
- `initial_count`: Starting number of prey

### World Parameters
- `width`: World width in units
- `height`: World height in units
- `boundary_type`: Wraparound vs walls
- `food_spawn_rate`: Rate of food source generation (optional)
- `food_energy`: Energy value of food sources

### Simulation Parameters
- `tick_rate`: Updates per second (simulation speed)
- `max_agents`: Maximum total agents (performance limit)
- `enable_reproduction`: Toggle reproduction on/off
- `enable_food`: Toggle food sources on/off

---

## Documentation Strategy

### User-Facing Documentation
1. **README.md**: Quick start, installation, basic usage
2. **USER_GUIDE.md**: 
   - How to use the UI
   - What each parameter does
   - Example scenarios
   - Interpreting results
   - Troubleshooting

3. **EXAMPLES.md**:
   - Classic Lotka-Volterra scenario
   - High predation rate scenario
   - Predator extinction scenario
   - Stable ecosystem scenario

### Developer Documentation
1. **ARCHITECTURE.md**:
   - System overview
   - Component relationships
   - Design decisions
   - Extension points

2. **SIMULATION_MODEL.md**:
   - Mathematical model description
   - Agent behavior algorithms
   - Interaction rules
   - Edge cases

3. **DEVELOPER_GUIDE.md**:
   - Setting up development environment
   - Code organization
   - Adding new agent types
   - Contributing guidelines

4. **Code Documentation**:
   - Public API documentation (rustdoc)
   - Inline comments for complex logic
   - Examples in documentation
   - Algorithm explanations

### Educational Documentation
1. **CONCEPTS.md**:
   - Ecology concepts explained
   - Predator-prey dynamics
   - Agent-based modeling introduction
   - Real-world applications

---

## Development Phases

### Phase 1: Setup & Foundation (Week 1)
- Project structure setup
- Cargo.toml with dependencies
- Basic project skeleton
- Initial documentation structure

### Phase 2: Core Simulation (Week 2-3)
- Implement agent system
- Implement predator and prey
- Implement world management
- Basic movement and interaction
- Unit tests for core logic

### Phase 3: UI Foundation (Week 4)
- egui window setup
- Basic rendering canvas
- Simple agent visualization
- Control panel framework

### Phase 4: Full UI Integration (Week 5)
- All parameter sliders
- Statistics display
- Control buttons
- Real-time updates

### Phase 5: Advanced Features (Week 6)
- Reproduction mechanics
- Spatial optimization
- Performance tuning
- Save/load functionality

### Phase 6: Documentation (Week 7)
- Complete all documentation
- Code comments and rustdoc
- Example scenarios
- User guide with screenshots

### Phase 7: Polish & Testing (Week 8)
- Bug fixes
- Performance optimization
- UI/UX improvements
- Final testing

---

## Technical Considerations

### Performance
- Use spatial partitioning (grid or quadtree) for O(n) to O(n log n) collision detection
- Limit maximum agents to prevent performance degradation
- Efficient agent storage (Vec or similar)
- Consider multithreading for large simulations (optional)

### Extensibility
- Trait-based agent system for easy extension
- Plugin-style architecture for custom behaviors
- Modular parameter system
- Clear separation of concerns

### Educational Value
- Clear, readable code
- Well-commented algorithms
- Configurable difficulty/complexity
- Visual feedback for all actions
- Statistics that help understand dynamics

---

## Dependencies (Cargo.toml draft)

```toml
[package]
name = "predator-prey-sim"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = "0.24"
eframe = "0.24"  # egui framework
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
# Optional: for vector math
nalgebra = "0.32"

[dev-dependencies]
criterion = "0.5"  # For benchmarking
```

---

## Success Criteria

1. **Functionality**:
   - Stable simulation runs indefinitely
   - All core behaviors work correctly
   - UI is responsive and intuitive
   - Parameters have clear effects

2. **Performance**:
   - Smooth 60 FPS with 100+ agents
   - Real-time parameter updates
   - No memory leaks

3. **Documentation**:
   - Complete user documentation
   - Comprehensive code documentation
   - Clear examples and tutorials
   - Architecture explanation

4. **Educational Value**:
   - Easy to understand and use
   - Demonstrates key concepts clearly
   - Encourages experimentation
   - Provides learning insights

---

## Next Steps

1. Review and refine this plan
2. Set up project structure
3. Begin Phase 1 implementation
4. Iterate based on testing and feedback

---

## Notes & Considerations

- Start simple, add complexity gradually
- Prioritize clarity over optimization initially
- Focus on educational value in design decisions
- Keep UI intuitive and uncluttered
- Consider accessibility (keyboard navigation, screen reader support)
- Future expansion possibilities:
  - Multiple predator/prey species
  - Environmental factors (terrain, obstacles)
  - Genetic algorithms
  - Network effects
  - Export animations/videos

