# Old Heads - Physics-Based Basketball Game

## Project Description

**Vision**: A physics-based basketball game featuring older players where shooting and passing are genuinely difficult, rewarding careful playmaking and basketball IQ over raw athleticism. Successfully executing a play with your team that leads to a layup should feel amazing because it's earned through precision and strategy.

**Core Concept**:
- **All players are older men & women** - no young athletes, embracing the "old heads" basketball culture
- **No dunking** - plays are scored through layups, jump shots, and teamwork
- **No fast running** - slower, more deliberate pace that emphasizes positioning and decision-making
- **Good old-fashioned playmaking** - success comes from reading the defense, smart passing, and executing plays
- **Physics perfection is key** - tweaking physics and mechanics to make extremely difficult gameplay feel rewarding when mastered

**Control Philosophy**:
- **Manual dribbling** - use Q/E buttons to manually dribble left/right hand
- **FPS-style aiming** - passes don't lock on to teammates; you must aim the ball like aiming a gun in an FPS
- **No assists** - every action requires manual skill and precision
- **Unique and intentionally difficult** - control scheme designed to feel new and challenging, reinforcing physics mastery

**Technology Stack**:
- Bevy 0.18 game engine
- Rust programming language
- Avian3D 0.5 physics engine
- 3D graphics with PBR rendering

**Purpose**: Learning 3D game development with Bevy while creating a unique basketball game that stands apart from mainstream titles through physics-first difficulty, thematic constraints, and rewarding mastery.

**Inspiration**: This project is inspired by the **physics-first difficulty** found in games like Schedule 1 and Peak, which make shooting and passing genuinely challenging through realistic physics rather than assisted mechanics.

---

## Current Controls

| Input | Action |
|-------|--------|
| WASD | Move player (2.5 m/s walk speed) |
| Mouse | Look around (click to grab cursor) |
| Escape | Release cursor |
| F | Pick up ball (or auto-pickup when close) |
| Q | Dribble with left hand |
| E | Dribble with right hand |
| Left Mouse (hold) | Charge shot |
| Left Mouse (release) | Throw/shoot ball |
| Shift + Left Mouse | Pass (type based on camera angle) |
| T | Debug: Teleport ball to player |

**Pass Types** (determined by where you're looking):
- Looking down (~15°+) → Bounce pass
- Looking level → Chest pass
- Looking up (~12°+) → Lob pass

---

## Current Implementation

### Player Plugin (`src/player/`)

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

### Ball Plugin (`src/ball/`)

**Ball States**: `Free` → `Held` → `Dribbling` → `Free`

**Systems**:
- `ball_pickup` - Pick up ball with F key or auto-pickup when within range
- `ball_hold_position` - Keep ball positioned in front of camera when held
- `dribble_start` - Initiate dribbling with Q (left) or E (right)
- `dribble_input` - Continue dribbling or crossover to opposite hand
- `dribble_physics_update` - Animate ball bouncing during dribble
- `dribble_timeout_check` - Auto-pickup if no dribble input
- `shot_charge_start` - Begin charging when left mouse pressed
- `shot_charge_update` - Update charge level while holding
- `shot_charge_cancel` - Cancel charge on Escape or ball lost
- `shot_charge_indicator` - Draw charge bar using gizmos
- `ball_throw` - Release ball with velocity based on charge level

**Throw Mechanics**:
- Charge time: 1.0 second for full power
- Power range: 3.0 - 13.0 m/s
- Arc angle: 5° - 35° (added to camera pitch)
- Backspin: 2.0 - 8.0 rad/s
- Charge curve: S-Curve for precision at extremes

### Scoring Plugin (`src/scoring/`)

- **Hoop sensors** spawned at both ends of court
- **Score detection** using collision sensors
- **Debug teleport** (T key) to bring ball back to player

### Physics Config (`src/physics_config.rs`)

Centralized tunable parameters for all physics:
- Ball properties (radius, restitution, friction)
- Throw mechanics (power, charge time, arc angles, backspin)
- Dribble mechanics (timeout, cooldown, bounce height, offsets)
- Pickup/hold settings (range, hold offset)

---

## Project Structure

```
src/
├── main.rs                 # App setup, spawns light/court/basketballs
├── physics_config.rs       # Centralized physics tuning parameters
├── ball/
│   ├── mod.rs
│   ├── plugin.rs           # BallPlugin definition
│   ├── components.rs       # Basketball, BallState, HeldBy, DribbleState, Hand
│   ├── resources.rs        # ShootingChargeState
│   └── systems/
│       ├── mod.rs
│       ├── pickup.rs       # Ball pickup logic
│       ├── hold.rs         # Ball hold positioning
│       ├── dribble.rs      # Dribble mechanics
│       ├── charge.rs       # Shot charging system
│       └── throw.rs        # Ball throwing/shooting
├── scoring/
│   ├── mod.rs
│   ├── plugin.rs           # ScoringPlugin definition
│   ├── components.rs       # HoopSensor markers
│   ├── resources.rs        # Score resource
│   └── systems.rs          # Scoring detection, debug teleport
└── player/
    ├── mod.rs
    ├── plugin.rs           # PlayerPlugin definition
    ├── components.rs       # Player, CameraMount, FpsHands
    ├── resources.rs        # HandAnimations, PlayerSettings, CursorGrabbed
    └── systems/
        ├── mod.rs
        ├── spawn.rs        # Player entity hierarchy spawning
        ├── movement.rs     # WASD input and velocity
        └── camera.rs       # Mouse look, cursor grab
```

---

## Character Animation (Blender)

**Rig**: `basicRig` - Arm/hand rig with IK constraints
- 50 bones total (both arms, all fingers)
- Arms use **Inverse Kinematics** - control via `forearm.L.001` / `forearm.R.001` location
- Hands and fingers use direct rotation

**Animations Created**:
| Action Name | Frames | Description |
|-------------|--------|-------------|
| RightHandDribble | 1-30 | Right hand dribbling motion |
| LeftHandDribble | 1-30 | Left hand dribbling motion (mirrored) |
| Shooting | 1-38 | Basketball shooting motion |

**Animation Documentation**:
- `docs/blender-mcp-animation-guide.md` - Full guide for creating animations via MCP
- `docs/BLENDER_MCP_PROMPT.md` - Concise prompt for AI agents
- `.claude/commands/blender-animate.md` - Slash command for animation tasks

---

## Milestones Completed

- [x] Ball bounces on floor with physics simulation
- [x] First-person player controller with WASD movement
- [x] Mouse look camera controls
- [x] FPS hands model integrated with animations
- [x] Street basketball court scene
- [x] Ball pickup/hold mechanics
- [x] Manual dribbling controls (Q/E two-button system)
- [x] Dribble physics with bounce animation
- [x] Crossover dribbling between hands
- [x] Shot charging system with visual indicator
- [x] Throwing/shooting with FPS-style aiming
- [x] Basketball hoops with scoring detection
- [x] Centralized physics config for tuning
- [x] Passing mechanics (right-click, camera angle determines pass type)

---

## Known Issues

- **Input lag with WASD + mouse**: Fixed by using `!pressed()` instead of `just_released()` for mouse release detection (Bevy input ordering issue with simultaneous keyboard/mouse)

---

## TODO

### Immediate
- [ ] Fine-tune physics for better "Old Heads" feel
- [ ] Connect hand animations to ball state (dribble animations, shooting animation)
- [ ] Score display UI

### Future Features
- [ ] Player character body model (older man/woman)
- [ ] Walking/running animations
- [ ] AI opponents
- [ ] Sound effects (ball bounce, swish, etc.)
- [ ] Multiple game modes

---

## Session Log

**2026-01-26**: Fixed input state persistence bug where shot charge would persist when holding WASD + releasing mouse. Changed from `just_released()` to `!pressed()` for reliable mouse release detection. Reduced ball count from 50 to 5 for performance testing.

**2026-01-26**: Changed pass controls from keyboard (Z/X/C then 1/2/3) to Shift+click with camera angle determining pass type. This avoids keyboard ghosting issues when holding Q+W for dribble+move. Looking down = bounce pass, level = chest pass, up = lob pass. Created GitHub repo `glass-waves/old_heads`.
