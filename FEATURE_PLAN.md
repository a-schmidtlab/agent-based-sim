# Feature Implementation Plan: Torus Topography & Live Evaluation

## Overview
This plan details the implementation of three major features:
1. Torus topography (wraparound world)
2. Interactive initial population sliders with live spawn
3. Live evaluation graphics (population graphs and statistics)

---

## 1. Torus Topography Enhancement

### Current State
- `BoundaryType::Wraparound` exists but may not be properly enforced
- Distance calculations need to account for torus geometry
- Spatial queries should consider wraparound

### Implementation Tasks
1. **Fix Distance Calculation for Torus**
   - Implement `distance_torus()` function that calculates shortest path on torus
   - Account for wrapping in both X and Y axes
   - Update agent perception/detection to use torus distance

2. **Enforce Torus as Default/Only Mode**
   - Set `BoundaryType::Wraparound` as default and preferred
   - Ensure all position updates properly wrap
   - Verify edge cases (agents at exact boundaries)

3. **Visual Indication**
   - Optional: Visual markers showing torus connectivity (edge indicators)
   - Agent teleportation visualization at boundaries

### Files to Modify
- `src/utils/math.rs` - Add `distance_torus()` function
- `src/simulation/world.rs` - Update spatial queries to use torus distance
- `src/simulation/agent.rs` - Verify wrapping works correctly
- `src/config/parameters.rs` - Set wraparound as default

### Tests Required
- Test distance calculation across boundaries
- Test agent movement across edges
- Test perception/detection at boundaries
- Integration test for full torus simulation

---

## 2. Interactive Initial Population Controls

### Current State
- Initial counts exist in parameters but are only used at startup
- Sliders exist but don't spawn agents dynamically

### Implementation Tasks
1. **Add Spawn Controls to UI**
   - Prominent sliders for predator/prey initial counts (0-500 range)
   - "Spawn Now" buttons to add agents to running simulation
   - "Clear All" button option
   - Real-time display of current counts

2. **Dynamic Agent Spawning**
   - `World::spawn_predators(count: u32)` method
   - `World::spawn_prey(count: u32)` method
   - Random positioning within world bounds
   - Respect max_agents limit

3. **Reset Integration**
   - Reset button should use current slider values
   - Preserve slider values across resets

### Files to Modify
- `src/simulation/world.rs` - Add spawn methods
- `src/ui/controls.rs` - Add spawn controls UI
- `src/main.rs` - Handle spawn requests

### Tests Required
- Test spawning predators/prey individually
- Test spawning respects max_agents
- Test spawn positioning is within bounds
- Test reset uses current slider values

---

## 3. Live Evaluation Graphics

### Current State
- Basic statistics window exists
- No historical data tracking
- No graphical visualization of trends

### Implementation Tasks
1. **Data Collection System**
   - Create `StatisticsCollector` struct
   - Track population over time (predator count, prey count)
   - Track average energy levels
   - Configurable history length (e.g., last 1000 ticks)
   - Circular buffer for efficient storage

2. **Graph Visualization**
   - Line graphs showing:
     - Predator population over time (red line)
     - Prey population over time (green line)
     - Both on same graph for comparison
   - X-axis: Time/Ticks
   - Y-axis: Population count
   - Auto-scaling based on data range

3. **Statistics Window Enhancement**
   - Current values (predators, prey, total)
   - Historical stats:
     - Peak populations
     - Current trend (increasing/decreasing)
     - Average populations
   - Energy statistics:
     - Average predator energy
     - Average prey energy
   - Optional: Extinction warnings

4. **Graph Controls**
   - Clear history button
   - Pause/resume data collection
   - Zoom controls
   - Export data button (future)

### Files to Create/Modify
- `src/ui/statistics.rs` - New module for statistics collection and graphing
- `src/main.rs` - Integrate statistics collector
- `src/ui/visualization.rs` - Add graph rendering functions
- `src/simulation/world.rs` - Expose energy averages

### Tests Required
- Test data collection accuracy
- Test circular buffer overflow handling
- Test graph rendering with various data
- Test statistics calculations
- Test reset/clear functionality

---

## Implementation Order

### Phase 1: Torus Topography (Priority 1)
1. Implement `distance_torus()` function
2. Update world spatial queries
3. Add comprehensive tests
4. Verify all edge cases

### Phase 2: Interactive Population Controls (Priority 2)
1. Add spawn methods to World
2. Add UI controls
3. Integrate with main app
4. Add tests

### Phase 3: Live Evaluation Graphics (Priority 3)
1. Create StatisticsCollector
2. Implement graph rendering
3. Enhance statistics window
4. Add tests
5. Polish UI

---

## Testing Strategy

### Unit Tests
- Torus distance calculations
- Spawn functions
- Statistics collection
- Data structure operations

### Integration Tests
- Full simulation with torus boundaries
- Spawn during running simulation
- Statistics collection over long runs
- Reset with new population counts

### Visual Tests
- Manual testing of UI responsiveness
- Graph rendering quality
- Performance with large populations

---

## Documentation Updates

### README.md Updates
- Feature list with new capabilities
- Usage instructions for new features
- Screenshots/examples of graphs
- Parameter explanations

### Code Documentation
- Document torus distance algorithm
- Document spawn API
- Document statistics collector API

---

## Success Criteria

1. ✅ Agents correctly wrap around world boundaries
2. ✅ Distance calculations work across torus edges
3. ✅ Users can spawn agents dynamically during simulation
4. ✅ Population graphs display correctly and update in real-time
5. ✅ Statistics window shows meaningful data
6. ✅ All tests pass
7. ✅ Documentation is comprehensive and clear

---

## Estimated Implementation Time
- Torus: 2-3 hours
- Spawn controls: 1-2 hours
- Live graphs: 3-4 hours
- Testing: 2 hours
- Documentation: 1 hour
**Total: ~10-12 hours**

