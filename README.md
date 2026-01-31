# old heads

a silly game about playing basketball when you are old.

built with bevy 0.18 + avian3d physics.

## controls

| input | action |
|-------|--------|
| wasd | move player |
| mouse | look around (click to grab cursor) |
| escape | release cursor |
| f | pick up ball |
| q | dribble with left hand |
| e | dribble with right hand |
| left mouse (hold) | charge shot |
| left mouse (release) | shoot ball |
| shift + left mouse | pass |
| t | debug: teleport ball to player |

**pass types** are determined by camera angle:
- looking down → bounce pass
- looking level → chest pass
- looking up → lob pass


## todos
- fix rim colliders
- add net animation for score
- score ui
- game logic for scoring
- court zones for scoring
- first person arm model with animation
- sound effects
- defensive off-ball moves (steal, block)
- offensive off-ball moves (set screen, call for ball)
- multiplayer networking
- proximity chat with spatialized audio
- game logic for scoring
- timer
- fun outfits
- lots more
