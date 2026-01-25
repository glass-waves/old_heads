# Avian3D Physics Integration with Bevy

> **Current versions**: Bevy 0.18, Avian3D 0.5

## Overview

Avian3D physics integrates seamlessly with Bevy's ECS (Entity Component System) by registering systems that run automatically every frame. Physics components are just regular Bevy components, and the physics engine handles updating positions, velocities, and collision responses.

## Bevy's Frame Loop Structure

Every frame, Bevy runs through multiple stages in a fixed order:

```
┌─────────────────────────────────────┐
│ 1. First                             │ (Rarely used)
├─────────────────────────────────────┤
│ 2. PreUpdate                         │ Input handling, events
├─────────────────────────────────────┤
│ 3. Update                            │ ← Your game logic
├─────────────────────────────────────┤
│ 4. PostUpdate                        │ Transforms, cleanup
├─────────────────────────────────────┤
│ 5. Last                              │ (Rarely used)
├─────────────────────────────────────┤
│ 6. Render                            │ Extract → Prepare → Render
└─────────────────────────────────────┘
```

## What PhysicsPlugins Adds

When you call `.add_plugins(PhysicsPlugins::default())`, it registers multiple system sets that run at specific points in the frame:

### PhysicsSet::Prepare (PreUpdate)
- Clear collision events from last frame
- Wake up sleeping objects if needed
- Prepare physics state for the new frame

### PhysicsSet::StepSimulation (Update)
The main physics simulation runs here in this order:

1. **Integrate Forces**
   - Apply gravity (default: -9.81 m/s² on Y axis)
   - Apply external forces/impulses from your game logic
   - Update velocities based on forces

2. **Detect Collisions**
   - Check all colliders for overlaps
   - Generate collision pairs
   - Determine contact points and normals

3. **Solve Constraints**
   - Resolve collisions (push objects apart)
   - Apply restitution (bounce)
   - Apply friction
   - Solve joints/constraints

4. **Update Velocities**
   - Apply constraint results to velocities
   - Handle collision responses

5. **Integrate Velocities**
   - Update positions based on final velocities
   - Physics now has new positions internally

### PhysicsSet::Sync (PostUpdate)
**Critical step: Copy physics state to visual components**

- Reads internal physics positions
- Writes to `Transform.translation`
- Writes to `Transform.rotation`
- This is why you see objects move on screen!

## The Sync Process: Physics ↔ Visuals

This is the key to understanding how it all works together:

```rust
// Your basketball entity has BOTH:
Transform::from_xyz(0.0, 3.0, 0.0)  // Visual position (what you see)
RigidBody::Dynamic                   // Physics simulation state
```

**How sync works:**

```
┌─────────────────────────────────────────────────────────┐
│ Physics Engine                                          │
│ ↓                                                       │
│ Calculates new position: (0.0, 2.9, 0.0)              │
│ Stores in internal physics data structures             │
└─────────────────────────────────────────────────────────┘
                    ↓ Sync System
┌─────────────────────────────────────────────────────────┐
│ Transform Component                                     │
│ ↓                                                       │
│ Transform.translation = (0.0, 2.9, 0.0)                │
└─────────────────────────────────────────────────────────┘
                    ↓ Render System
┌─────────────────────────────────────────────────────────┐
│ Rendering                                               │
│ ↓                                                       │
│ Draws mesh at Transform.translation                    │
│ User sees ball at (0.0, 2.9, 0.0)                      │
└─────────────────────────────────────────────────────────┘
```

## Concrete Example: Bouncing Basketball

Let's trace what happens frame-by-frame:

### Frame 1 (Ball just spawned at y=3.0)

```
PreUpdate:
  PhysicsSet::Prepare
    - Clear events
    - Ball is awake (dynamic rigid body)

Update:
  PhysicsSet::StepSimulation
    1. Integrate Forces
       - Apply gravity: velocity.y = -9.81 * dt
       - Ball velocity = (0.0, -0.098, 0.0) [assuming 60fps]

    2. Detect Collisions
       - Check ball collider vs floor collider
       - No collision yet (ball at y=3.0, floor at y=0.0)

    3. Solve Constraints
       - No collisions to resolve

    4. Update Velocities
       - Velocity unchanged: (0.0, -0.098, 0.0)

    5. Integrate Velocities
       - New position = old_pos + velocity * dt
       - Position = (0.0, 3.0, 0.0) + (0.0, -0.098, 0.0)
       - Position = (0.0, 2.902, 0.0)

PostUpdate:
  PhysicsSet::Sync
    - Transform.translation = (0.0, 2.902, 0.0)

Render:
  - Draw ball mesh at (0.0, 2.902, 0.0)
  - User sees ball slightly lower than before
```

### Frame 30 (Ball hits floor at y≈0.0)

```
Update:
  PhysicsSet::StepSimulation
    1. Integrate Forces
       - Gravity applied (velocity now very negative)
       - velocity.y ≈ -2.94 m/s (falling fast)

    2. Detect Collisions
       - Ball sphere collider (radius 0.12) at y=0.12
       - Floor cuboid collider at y=0.0
       - COLLISION DETECTED! ✓
       - Contact point: (0.0, 0.0, 0.0)
       - Contact normal: (0.0, 1.0, 0.0) [pointing up]

    3. Solve Constraints
       - Push ball up to separate from floor
       - Calculate bounce velocity using Restitution(0.8)
       - new_velocity.y = -old_velocity.y * 0.8
       - new_velocity.y = -(-2.94) * 0.8 = +2.352 m/s
       - Apply friction (ball/floor contact)

    4. Update Velocities
       - Velocity = (0.0, +2.352, 0.0) [bouncing upward!]

    5. Integrate Velocities
       - Position = (0.0, 0.12, 0.0) [slightly above floor]

PostUpdate:
  PhysicsSet::Sync
    - Transform.translation = (0.0, 0.12, 0.0)

Render:
  - Draw ball at (0.0, 0.12, 0.0)
  - User sees ball bounce!
```

### Next Frames (Ball bouncing up)

```
Update:
  - Gravity pulls ball down, slowing upward velocity
  - velocity.y decreases: +2.352 → +2.254 → +2.156 → ...
  - Ball rises until velocity.y = 0
  - Then falls again
  - Each bounce loses 20% energy (restitution = 0.8)
  - Eventually settles on floor
```

## Component Relationships

### Dynamic Rigid Body (Basketball)
```rust
commands.spawn((
    // Visual components (what you see)
    Mesh3d(meshes.add(Sphere::new(0.12))),
    MeshMaterial3d(materials.add(Color::srgb(0.9, 0.4, 0.1))),
    Transform::from_xyz(0.0, 3.0, 0.0),

    // Physics components (simulation)
    RigidBody::Dynamic,        // Affected by gravity & forces
    Collider::sphere(0.20),    // Collision shape
    Restitution::new(0.95),    // Bounciness (95% energy retained)
    Friction::new(0.3),        // Surface friction
));
```

**Physics updates:** Position, velocity
**Sync writes to:** Transform.translation

### Static Rigid Body (Floor)
```rust
commands.spawn((
    // Visual components
    Mesh3d(...),
    MeshMaterial3d(...),

    // Physics components
    RigidBody::Static,         // Never moves
    Collider::cuboid(...),     // Collision shape
    Friction::new(0.5),        // Surface friction
));
```

**Physics updates:** Nothing (static = never moves)
**Sync writes to:** Nothing (no need, position never changes)

## Time Stepping

### Variable Timestep (Default)
- Physics runs once per frame
- Timestep = frame delta time
- Simple, but physics depends on framerate
- 60fps vs 30fps will have different results

### Fixed Timestep (Optional)
```rust
PhysicsPlugins::default()
    .with_fixed_timestep(1.0 / 60.0)  // 60Hz physics regardless of FPS
```
- Physics runs at fixed rate (e.g., 60Hz)
- Multiple substeps if frame is slow
- Deterministic, good for multiplayer
- Slightly more complex

### Substeps (For Stability)
```rust
PhysicsPlugins::default()
    .with_substeps(4)  // Run physics 4x per frame
```
- Breaks each physics step into smaller steps
- More accurate collision detection
- Prevents tunneling (fast objects passing through)
- More expensive

## System Ordering Example

```rust
// Simplified view of what Avian does internally:
app
    .configure_sets(
        Update,
        (
            PhysicsSet::Prepare,
            PhysicsSet::StepSimulation,
            PhysicsSet::PostProcessCollisions,
        )
        .chain()  // Run in order
    )
    .configure_sets(
        PostUpdate,
        PhysicsSet::Sync  // After all physics done
    )

    // Your game systems can order relative to physics:
    .add_systems(
        Update,
        my_system.before(PhysicsSet::StepSimulation)
    );
```

## Common Patterns

### Applying Forces
```rust
fn apply_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut velocity in &mut query {
            velocity.y = 5.0;  // Jump!
        }
    }
}

// Run BEFORE physics so it takes effect this frame
app.add_systems(Update, apply_jump.before(PhysicsSet::StepSimulation));
```

### Reading Collisions
```rust
fn detect_scoring(
    mut collision_events: EventReader<Collision>,
) {
    for Collision(contacts) in collision_events.read() {
        // Check if ball hit hoop
        // Award points
    }
}

// Run AFTER physics generates collision events
app.add_systems(Update, detect_scoring.after(PhysicsSet::StepSimulation));
```

### Moving Kinematic Bodies
```rust
fn move_platform(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Platform>>,
) {
    for mut transform in &mut query {
        transform.translation.x = time.elapsed_secs().sin() * 5.0;
    }
}

// Kinematic bodies moved by Transform, physics follows
app.add_systems(Update, move_platform.before(PhysicsSet::Sync));
```

## Performance Considerations

### What Avian Does Every Frame
- Broadphase collision detection (fast culling)
- Narrowphase collision detection (precise checks)
- Constraint solving (iterative process)
- Position updates

### Optimization Tips
1. **Use sleeping** - Static objects automatically sleep when not moving
2. **Simplify colliders** - Use simple shapes (sphere, box) when possible
3. **Collision layers** - Prevent unnecessary checks between object types
4. **Reduce substeps** - Only use what you need for stability

## Debugging

### Visualize Colliders
```rust
PhysicsPlugins::default()
    .add_plugins(PhysicsDebugPlugin::default())
```
- Draws wireframe colliders
- Shows contact points
- Displays velocities

### Check Performance
```rust
app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
   .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());
```
- Monitor frame time
- Identify physics bottlenecks

## Summary

**Key Takeaways:**

1. **PhysicsPlugins adds systems** that run automatically every frame
2. **Physics runs in Update stage**, calculates new positions internally
3. **Sync runs in PostUpdate**, copies physics state → Transform
4. **Render uses Transform** to draw objects at correct positions
5. **You interact via components**: Add forces, read collisions, spawn entities
6. **Separation of concerns**: Physics engine doesn't know about rendering, renderer doesn't know about physics - Transform is the bridge

This design keeps physics deterministic and separate from rendering, while Bevy's ECS makes it feel seamless to use.
