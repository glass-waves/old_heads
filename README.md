# Old Heads

A physics-based basketball game where shooting and passing are genuinely difficult. All players are older men & women—no dunking, no fast running, just good old-fashioned playmaking.

Built with Bevy 0.18 + Avian3D physics.

## Controls

| Input | Action |
|-------|--------|
| WASD | Move player |
| Mouse | Look around (click to grab cursor) |
| Escape | Release cursor |
| F | Pick up ball |
| Q | Dribble with left hand |
| E | Dribble with right hand |
| Left Mouse (hold) | Charge shot |
| Left Mouse (release) | Shoot ball |
| Shift + Left Mouse | Pass |
| T | Debug: Teleport ball to player |

**Pass types** are determined by camera angle:
- Looking down → Bounce pass
- Looking level → Chest pass
- Looking up → Lob pass

## Current State

Working:
- First-person player with FPS hands model
- Ball pickup, holding, and manual dribbling (Q/E)
- Shot charging with power indicator
- Throwing/shooting with FPS-style aiming (no aim assist)
- Passing system (shift+click, angle-based)
- Basketball hoops with score detection
- Street court environment

Not yet implemented:
- Hand animations connected to ball state
- Score UI
- Player body model
- AI opponents
- Sound effects

## Running

```bash
cargo run
```

## Project Structure

- `src/player/` - First-person controller, camera, movement
- `src/ball/` - Ball states, dribbling, charging, throwing
- `src/scoring/` - Hoop sensors, score detection
- `src/physics_config.rs` - Tunable physics parameters
