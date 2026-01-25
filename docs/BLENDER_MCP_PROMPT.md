# Blender MCP Animation Agent Instructions

You have access to Blender via MCP tools. Follow these instructions when creating animations.

## Available Tools

- `mcp__blender__get_scene_info` - Get scene overview (objects, types, locations)
- `mcp__blender__get_object_info` - Get specific object details
- `mcp__blender__get_viewport_screenshot` - Capture viewport (use max_size=800)
- `mcp__blender__execute_blender_code` - Run Python code in Blender

## Workflow: Creating Animations

### Phase 1: Exploration (REQUIRED)

1. **Get scene info** to understand what objects exist
2. **Take screenshot** to visually understand the scene
3. **List armature bones** if animating a character:

```python
import bpy
armature = bpy.data.objects['ARMATURE_NAME']
for bone in sorted([b.name for b in armature.pose.bones]):
    print(f"  - {bone}")
```

4. **Check for IK constraints** (IMPORTANT - changes how you animate):

```python
import bpy
armature = bpy.data.objects['ARMATURE_NAME']
for bone in armature.pose.bones:
    if bone.constraints:
        for c in bone.constraints:
            if c.type == 'IK':
                print(f"{bone.name} has IK → target: {c.subtarget}")
```

### Phase 2: Determine Control Method

**If bones have IK constraints**: Control via IK target LOCATION (not rotation)
**If no IK**: Control via direct bone ROTATION

#### IK Rig Pattern (Common for Arms)
```
upper_arm.R    → Auto-controlled by IK solver
forearm.R      → Auto-controlled by IK solver
forearm.R.001  → IK TARGET - move this with LOCATION keyframes
hand.R         → Direct rotation control
fingers        → Direct rotation control
```

### Phase 3: Animation Creation

#### Template A: Direct Rotation (Non-IK bones)

```python
import bpy
import math
from mathutils import Euler

ARMATURE_NAME = "your_armature_name"
ACTION_NAME = "YourAnimationName"

armature = bpy.data.objects[ARMATURE_NAME]
pose_bones = armature.pose.bones

# Setup action
bpy.context.view_layer.objects.active = armature
bpy.ops.object.mode_set(mode='POSE')
action = bpy.data.actions.new(name=ACTION_NAME)
action.use_fake_user = True  # IMPORTANT: Protects action from deletion
armature.animation_data_create()
armature.animation_data.action = action
bpy.context.scene.frame_start = 1
bpy.context.scene.frame_end = 30

# Helper: Rotation keyframe
def keyframe_rot(bone_name, frame, rotation_degrees):
    bone = pose_bones[bone_name]
    bone.rotation_mode = 'XYZ'
    bone.rotation_euler = Euler((
        math.radians(rotation_degrees[0]),
        math.radians(rotation_degrees[1]),
        math.radians(rotation_degrees[2])
    ), 'XYZ')
    bone.keyframe_insert(data_path="rotation_euler", frame=frame)

# CREATE KEYFRAMES
keyframe_rot("hand.R", 1, (20, -23, -73))
keyframe_rot("hand.R", 15, (6, -6, -47))
keyframe_rot("hand.R", 30, (20, -23, -73))  # Match frame 1 for loop

print(f"Animation '{ACTION_NAME}' created")
```

#### Template B: IK Target Location (For IK-controlled arms)

```python
import bpy
from mathutils import Vector

armature = bpy.data.objects['ARMATURE_NAME']
pose_bones = armature.pose.bones

# Helper: Location keyframe for IK target
def keyframe_loc(bone_name, frame, location):
    bone = pose_bones[bone_name]
    bone.location = Vector(location)
    bone.keyframe_insert(data_path="location", frame=frame)

# Move IK target to control arm position
keyframe_loc("forearm.R.001", 1, (2.19, -0.37, 0.15))   # Hand up
keyframe_loc("forearm.R.001", 15, (2.43, -1.89, -0.86)) # Hand down
keyframe_loc("forearm.R.001", 30, (2.19, -0.37, 0.15))  # Loop
```

### Phase 4: Verification (REQUIRED)

**NEVER skip verification. NEVER declare complete without these checks.**

1. **Verify keyframe data**:
```python
import bpy, math
armature = bpy.data.objects['ARMATURE_NAME']
bone = armature.pose.bones["BONE_NAME"]
for frame in [1, 15, 30]:
    bpy.context.scene.frame_set(frame)
    print(f"Frame {frame}: rot={[round(math.degrees(x),1) for x in bone.rotation_euler]}, loc={[round(x,2) for x in bone.location]}")
```

2. **Visual verification** - Take screenshots at frame 1 AND middle frame
3. **Compare screenshots** - Confirm visible difference between poses

## Reference Values (Basketball Animations)

### IK Target Ranges (forearm.X.001 location)

| Motion | X | Y | Z |
|--------|---|---|---|
| Dribble UP | ±2.2 | -0.4 | 0.15 |
| Dribble DOWN | ±2.4 | -1.9 | -0.9 |
| Shooting START | ±2.2 | -0.4 | 0.15 |
| Shooting RELEASE | ±3.7 | 6.1 | 1.3 |

### Hand Rotation Ranges (degrees)

| Pose | X | Y | Z |
|------|---|---|---|
| Neutral/Catch | 20 | ±23 | ±73 |
| Push Down | 6 | ±6 | ±47 |
| Shooting Release | 84 | -14 | 80 |

### Finger Curl (X rotation only)

| Pose | Degrees |
|------|---------|
| Spread/Open | -15° |
| Neutral | 0° to 5° |
| Cupped | 20° to 45° |

## Mirroring Left ↔ Right

When mirroring animations:
- **IK Location X**: Negate
- **IK Location Y, Z**: Keep same
- **Hand Rotation X**: Keep same
- **Hand Rotation Y, Z**: Negate
- **Finger Rotation**: Keep same
- **Thumb Y Rotation**: Negate

## Key Rules

### Action Protection
```python
action.use_fake_user = True  # ALWAYS set this or action may be deleted
```

### Rotation Values
- Always use `math.radians()` - Blender uses radians internally
- X axis = flex/extend (curl, bend)
- Y axis = twist
- Z axis = spread/splay

### Bone Naming
- `.L` / `.R` = Left / Right
- `.001` suffix often = IK target
- `.01`, `.02`, `.03` = Joint chain (1=base, 3=tip)

### Looping Animations
- **Frame 1 MUST equal last frame** for seamless loops

### Common Mistakes
- Forgetting `import math`
- Not setting `rotation_mode = 'XYZ'`
- Rotating IK-controlled bones directly (use IK target location instead)
- Not setting `use_fake_user = True` on actions
- Declaring done without visual verification

## Viewport Issues

If model not visible:
```python
import bpy
bpy.ops.object.mode_set(mode='OBJECT')
bpy.ops.object.select_all(action='DESELECT')
armature = bpy.data.objects['ARMATURE_NAME']
armature.select_set(True)
bpy.context.view_layer.objects.active = armature
for area in bpy.context.screen.areas:
    if area.type == 'VIEW_3D':
        with bpy.context.temp_override(area=area, region=area.regions[-1]):
            bpy.ops.view3d.view_selected()
        break
bpy.ops.object.mode_set(mode='POSE')
```

## Verification Checklist

Before declaring complete:
- [ ] Checked for IK constraints and used appropriate control method
- [ ] Set `use_fake_user = True` on action
- [ ] Printed bone values at key frames
- [ ] Values change between frames as expected
- [ ] Screenshot at frame 1 taken
- [ ] Screenshot at middle frame taken
- [ ] Visual difference confirmed
- [ ] Loop points match (if looping)

## Exporting for Bevy

Export animations to GLB format for use in Bevy:

1. **Export Settings**: File → Export → glTF 2.0 (.glb)
   - Include: Armatures, Mesh, Animations
   - Animation mode: Actions (exports all actions)

2. **In Bevy** (example from this project):
```rust
// Load animations by index
let dribble_right = asset_server
    .load(GltfAssetLabel::Animation(0).from_asset("model.glb"));
let dribble_left = asset_server
    .load(GltfAssetLabel::Animation(1).from_asset("model.glb"));

// Create animation graph
let (graph, node_indices) = AnimationGraph::from_clips([
    dribble_right,
    dribble_left,
]);

// Play animation
animation_player.play(node_indices[0]).repeat();
```

**Note**: Animation indices in Bevy correspond to export order in Blender. Use `stop_all()` before switching animations for clean transitions.
