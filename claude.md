# Old Heads - Physics-Based Basketball Game

## Project Description

**Vision**: A physics-based basketball game featuring older players where shooting and passing are genuinely difficult, rewarding careful playmaking and basketball IQ over raw athleticism. Successfully executing a play with your team that leads to a layup should feel amazing because it's earned through precision and strategy.

**Core Concept**:
- **All players are older men** - no young athletes, embracing the "old heads" basketball culture
- **No dunking** - plays are scored through layups, jump shots, and teamwork
- **No fast running** - slower, more deliberate pace that emphasizes positioning and decision-making
- **Good old-fashioned playmaking** - success comes from reading the defense, smart passing, and executing plays
- **Physics perfection is key** - tweaking physics and mechanics to make extremely difficult gameplay feel rewarding when mastered

**Control Philosophy**:
- **Manual dribbling** - use two buttons to manually dribble, no automatic ball handling
- **FPS-style aiming** - passes don't lock on to teammates; you must aim the ball like aiming a gun in an FPS
- **No assists** - every action requires manual skill and precision
- **Unique and intentionally difficult** - control scheme designed to feel new and challenging, reinforcing physics mastery

**Technology Stack**:
- Bevy 0.18 game engine
- Rust programming language
- Avian3D 0.5 physics engine
- 3D graphics with PBR rendering

**Purpose**: Learning 3D game development with Bevy while creating a unique basketball game that stands apart from mainstream titles through physics-first difficulty, thematic constraints, and rewarding mastery.

**Inspiration**: This project is inspired by the **physics-first difficulty** found in games like Schedule 1 and Peak, which make shooting and passing genuinely challenging through realistic physics rather than assisted mechanics. Unlike mainstream basketball games where shooting is easy and almost secondary, this game aims to make shooting and passing **really hard** - success comes from mastering the physics, not from game assists.

## Current State & Tasks

### Current Implementation

**Player Plugin** (`src/player/`):
- **First-person camera** attached to player entity
- **FPS hands model** (`assets/first_person_model/fps-hands.glb`) with animations
- **WASD movement** at 2.5 m/s (slow "old heads" walk speed)
- **Mouse look** - click to grab cursor, Escape to release
- **Kinematic physics body** with capsule collider

**Entity Hierarchy**:
```
Player (kinematic rigid body, yaw rotation)
└── CameraMount (pitch rotation)
    ├── Camera3d
    └── FPS Hands Scene (with AnimationPlayer)
```

**Hand Animations** (debug keys 1-4):
| Key | Animation | Description |
|-----|-----------|-------------|
| 1 | Idle | Static pose (paused dribble) |
| 2 | DribbleRight | Right hand dribbling (looping) |
| 3 | DribbleLeft | Left hand dribbling (looping) |
| 4 | Shooting | Basketball shot (one-shot) |

**Scene Setup:**
- **Court**: Street basketball court GLB model (`assets/street_basketball_court.glb`)
- **Lighting**: DirectionalLight with illuminance of 5000.0
- **Architecture**: Using Bevy 0.18 required components system

**Physics Integration:**
- **Avian3D 0.5** physics engine (pure Rust, ECS-first design)
- **PhysicsPlugins** running every frame
- Gravity: -9.81 m/s² on Y axis

**Basketball:**
- **Orange sphere mesh** (radius 0.12m)
- **Dynamic rigid body** affected by gravity and collisions
- **Sphere collider** (radius 0.20m)
- **Physics properties**:
  - Restitution: 0.95 (bouncy)
  - Friction: 0.3

### Project Structure

```
src/
├── main.rs                 # App setup, spawns light/court/basketball
└── player/
    ├── mod.rs              # Module exports
    ├── plugin.rs           # PlayerPlugin definition
    ├── components.rs       # Player, CameraMount, FpsHands, etc.
    ├── resources.rs        # HandAnimations, PlayerSettings, CursorGrabbed
    ├── animation_state.rs  # HandAnimationState enum
    └── systems/
        ├── mod.rs
        ├── spawn.rs        # Player entity hierarchy spawning
        ├── movement.rs     # WASD input and velocity
        ├── camera.rs       # Mouse look, cursor grab
        └── animation.rs    # Animation state machine
```

### Character Animation (Blender)

**Rig**: `basicRig` - Arm/hand rig with IK constraints
- 50 bones total (both arms, all fingers)
- Arms use **Inverse Kinematics** - control via `forearm.L.001` / `forearm.R.001` location
- Hands and fingers use direct rotation

**Animations Created:**
| Action Name | Frames | Description |
|-------------|--------|-------------|
| RightHandDribble | 1-30 | Right hand dribbling motion |
| LeftHandDribble | 1-30 | Left hand dribbling motion (mirrored) |
| Shooting | 1-38 | Basketball shooting motion |

**Animation Documentation:**
- `docs/blender-mcp-animation-guide.md` - Full guide for creating animations via MCP
- `docs/BLENDER_MCP_PROMPT.md` - Concise prompt for AI agents
- `.claude/commands/blender-animate.md` - Slash command for animation tasks

**Export Format**: glTF/GLB for Bevy import

### Milestones Completed

- [x] Ball bounces on floor with physics simulation
- [x] First-person player controller with WASD movement
- [x] Mouse look camera controls
- [x] FPS hands model integrated with animations
- [x] Animation state machine (idle, dribble left/right, shooting)
- [x] Street basketball court scene

### Next Steps

**Immediate Tasks:**
- [ ] Fine-tune physics for "Old Heads" feel
- [ ] Implement ball pickup/hold mechanics
- [ ] Connect dribbling animations to actual ball control
- [ ] Add ability to throw/shoot ball with mouse aiming

**Future Features:**
- [ ] Basketball hoop with scoring detection
- [ ] Manual dribbling controls (two-button system)
- [ ] Player character body model (older man)
- [ ] Passing mechanics with FPS-style aiming
- [ ] Additional animations (passing, catching, walking)
- [ ] AI opponents
