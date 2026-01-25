# Blender MCP Animation Guide for Coding Agents

## Overview

This document provides comprehensive instructions for creating animations in Blender using the Blender MCP (Model Context Protocol) integration. It is designed for AI coding agents to follow when creating character animations.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Understanding Blender MCP Tools](#understanding-blender-mcp-tools)
3. [Initial Scene Exploration](#initial-scene-exploration)
4. [Working with Armatures](#working-with-armatures)
5. [Animation Creation Process](#animation-creation-process)
6. [Verification Procedures](#verification-procedures)
7. [Common Patterns and Code Templates](#common-patterns-and-code-templates)
8. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Setup
- Blender must be running with the Blender MCP addon enabled
- The addon must have its server started (check the addon panel in Blender)
- The MCP connection must be active

### Verifying Connection
Always start by testing the connection:

```python
# Use: mcp__blender__get_scene_info
# Expected: JSON with scene name, object count, objects list
# If error about "Could not connect to Blender" - addon is not running
```

---

## Understanding Blender MCP Tools

### Core Tools

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `get_scene_info` | Get overview of all objects in scene | First step - understand what exists |
| `get_object_info` | Get details about specific object | When you need object properties |
| `get_viewport_screenshot` | Capture current 3D view | Visual verification of changes |
| `execute_blender_code` | Run arbitrary Python in Blender | All animation/manipulation work |

### Tool Parameters

**get_viewport_screenshot**
- `max_size`: Integer, max dimension in pixels (default 400, use 800 for detail)
- `user_prompt`: String describing why you're taking the screenshot

**execute_blender_code**
- `code`: Python code string to execute in Blender
- `user_prompt`: String describing the operation
- Returns: `{"result": "Code executed successfully: <stdout output>"}` or error

---

## Initial Scene Exploration

### Step 1: Get Scene Overview

```python
# First call: mcp__blender__get_scene_info
# This returns:
# - scene name
# - object_count
# - objects array with name, type, location for each
# - materials_count
```

### Step 2: Identify Object Types

Common Blender object types you'll encounter:
- **MESH**: Visible 3D geometry (the actual model you see)
- **ARMATURE**: Skeleton/rig for animation (contains bones)
- **CAMERA**: Scene camera
- **LIGHT**: Light sources
- **EMPTY**: Helper objects (often used as controllers)

### Step 3: Take Initial Screenshot

Always capture a screenshot to understand the visual state:

```python
# Call: mcp__blender__get_viewport_screenshot with max_size=800
# This lets you SEE what's in the scene
```

### Step 4: Investigate the Armature

If animating a character, you need to understand the rig structure:

```python
import bpy

armature = bpy.data.objects['ARMATURE_NAME']  # Replace with actual name
bones = armature.pose.bones

bone_list = []
for bone in bones:
    bone_list.append(bone.name)

print("Available bones:", len(bone_list))
for name in sorted(bone_list):
    print(f"  - {name}")
```

---

## Working with Armatures

### Bone Naming Conventions

Most rigs follow these patterns:
- `.L` / `.R` suffix = Left / Right side
- `.01`, `.02`, `.03` = Chain sequence (e.g., finger joints)
- Common bone names:
  - `hand.L`, `hand.R` - Wrist/hand control
  - `forearm.L`, `forearm.R` - Lower arm
  - `upper_arm.L`, `upper_arm.R` - Upper arm
  - `f_index.01.L` - First joint of index finger, left hand
  - `thumb.01.R` - First joint of thumb, right hand

### Accessing Pose Bones

```python
import bpy

armature = bpy.data.objects['ARMATURE_NAME']
pose_bones = armature.pose.bones

# Access specific bone
hand_bone = pose_bones["hand.R"]

# Set rotation mode (important!)
hand_bone.rotation_mode = 'XYZ'  # Use Euler angles

# Read current rotation
current_rotation = hand_bone.rotation_euler
print(f"X: {current_rotation.x}, Y: {current_rotation.y}, Z: {current_rotation.z}")
```

### Rotation Axes

Understanding which axis does what (varies by rig, but common patterns):
- **X rotation**: Typically flex/extend (curl fingers, bend wrist forward/back)
- **Y rotation**: Typically twist/rotate along the bone
- **Z rotation**: Typically spread/splay (fingers apart)

**IMPORTANT**: Always use `math.radians()` to convert degrees to radians:
```python
import math
rotation_in_radians = math.radians(45)  # 45 degrees
```

---

## Animation Creation Process

### Step 1: Clear Existing Animation (if needed)

```python
import bpy

armature = bpy.data.objects['ARMATURE_NAME']
if armature.animation_data:
    armature.animation_data_clear()
```

### Step 2: Create New Action

```python
import bpy

armature = bpy.data.objects['ARMATURE_NAME']

# Ensure armature is active and in pose mode
bpy.context.view_layer.objects.active = armature
bpy.ops.object.mode_set(mode='POSE')

# Create new action
action = bpy.data.actions.new(name="MyAnimationName")
armature.animation_data_create()
armature.animation_data.action = action

# Set frame range
bpy.context.scene.frame_start = 1
bpy.context.scene.frame_end = 30  # 30 frames = 1 second at 30fps
```

### Step 3: Create Keyframes

Use this helper function pattern:

```python
import bpy
import math
from mathutils import Euler

armature = bpy.data.objects['ARMATURE_NAME']
pose_bones = armature.pose.bones

def set_rotation_keyframe(bone_name, frame, rotation_euler):
    """
    Set a rotation keyframe on a bone.

    Args:
        bone_name: String name of the bone (e.g., "hand.R")
        frame: Integer frame number
        rotation_euler: Tuple of (x, y, z) rotation in RADIANS
    """
    bone = pose_bones[bone_name]
    bone.rotation_mode = 'XYZ'
    bone.rotation_euler = Euler(rotation_euler, 'XYZ')
    bone.keyframe_insert(data_path="rotation_euler", frame=frame)

# Example usage:
set_rotation_keyframe("hand.R", frame=1, rotation_euler=(math.radians(-30), 0, 0))
set_rotation_keyframe("hand.R", frame=15, rotation_euler=(math.radians(60), 0, 0))
set_rotation_keyframe("hand.R", frame=30, rotation_euler=(math.radians(-30), 0, 0))
```

### Step 4: Animate Multiple Bones

For complex animations, animate multiple bones together:

```python
import bpy
import math
from mathutils import Euler

armature = bpy.data.objects['ARMATURE_NAME']
pose_bones = armature.pose.bones

def set_rotation_keyframe(bone_name, frame, rotation_euler):
    bone = pose_bones[bone_name]
    bone.rotation_mode = 'XYZ'
    bone.rotation_euler = Euler(rotation_euler, 'XYZ')
    bone.keyframe_insert(data_path="rotation_euler", frame=frame)

# Frame 1 - Starting position
frame = 1
set_rotation_keyframe("hand.R", frame, (math.radians(-30), 0, 0))
set_rotation_keyframe("forearm.R", frame, (math.radians(-10), 0, 0))

# Animate all fingers in a loop
for finger in ["f_index", "f_middle", "f_ring", "f_pinky"]:
    set_rotation_keyframe(f"{finger}.01.R", frame, (math.radians(-15), 0, 0))
    set_rotation_keyframe(f"{finger}.02.R", frame, (math.radians(-10), 0, 0))
    set_rotation_keyframe(f"{finger}.03.R", frame, (math.radians(-10), 0, 0))

# Frame 15 - Middle position
frame = 15
set_rotation_keyframe("hand.R", frame, (math.radians(60), 0, 0))
# ... etc
```

### Step 5: Ensure Seamless Looping

For looping animations, **frame 1 and the last frame must have identical values**:

```python
# Frame 1 values
start_hand_rotation = (math.radians(-30), 0, 0)

# Frame 1
set_rotation_keyframe("hand.R", 1, start_hand_rotation)

# Frame 30 (loop point) - MUST match frame 1
set_rotation_keyframe("hand.R", 30, start_hand_rotation)
```

---

## Verification Procedures

### Always Verify Your Work

Never declare an animation complete without verification. Follow these steps:

### 1. Verify Keyframe Data Exists

```python
import bpy
import math

armature = bpy.data.objects['ARMATURE_NAME']
action = armature.animation_data.action

print(f"Action: {action.name}")

# Check bone values at different frames
hand_bone = armature.pose.bones["hand.R"]

for frame in [1, 8, 15, 22, 30]:
    bpy.context.scene.frame_set(frame)
    rot = hand_bone.rotation_euler
    print(f"Frame {frame}: X={math.degrees(rot.x):.1f}°")
```

### 2. Visual Verification with Screenshots

Take screenshots at key frames to visually confirm the animation:

```python
import bpy

# Set to frame 1
bpy.context.scene.frame_set(1)
print("Ready for frame 1 screenshot")
```

Then call `mcp__blender__get_viewport_screenshot`.

Repeat for the middle frame (e.g., frame 15) to see the contrast.

### 3. Verification Checklist

Before declaring an animation complete, confirm:

- [ ] Action was created with correct name
- [ ] All intended bones have keyframes
- [ ] Frame 1 and last frame match (for looping animations)
- [ ] Visual screenshots show expected poses at key frames
- [ ] Frame range is set correctly in scene settings

---

## Common Patterns and Code Templates

### Template: Basic Looping Animation

```python
import bpy
import math
from mathutils import Euler

# Configuration
ARMATURE_NAME = "basicRig"  # Change to your armature name
ACTION_NAME = "MyAnimation"
FRAME_COUNT = 30

# Setup
armature = bpy.data.objects[ARMATURE_NAME]
pose_bones = armature.pose.bones

# Clear existing animation
if armature.animation_data:
    armature.animation_data_clear()

# Create action
bpy.context.view_layer.objects.active = armature
bpy.ops.object.mode_set(mode='POSE')
action = bpy.data.actions.new(name=ACTION_NAME)
armature.animation_data_create()
armature.animation_data.action = action

# Set frame range
bpy.context.scene.frame_start = 1
bpy.context.scene.frame_end = FRAME_COUNT

# Helper function
def keyframe(bone_name, frame, rotation_degrees):
    """Set keyframe with rotation in degrees (converted to radians)."""
    bone = pose_bones[bone_name]
    bone.rotation_mode = 'XYZ'
    bone.rotation_euler = Euler((
        math.radians(rotation_degrees[0]),
        math.radians(rotation_degrees[1]),
        math.radians(rotation_degrees[2])
    ), 'XYZ')
    bone.keyframe_insert(data_path="rotation_euler", frame=frame)

# ========== DEFINE YOUR KEYFRAMES HERE ==========

# Frame 1 - Start position
keyframe("hand.R", 1, (-30, 0, 0))

# Frame 15 - Middle position
keyframe("hand.R", 15, (60, 0, 0))

# Frame 30 - End position (match frame 1 for loop)
keyframe("hand.R", 30, (-30, 0, 0))

# ================================================

print(f"Animation '{ACTION_NAME}' created with {FRAME_COUNT} frames")
```

### Template: Mirror Animation to Other Side

To create a left-hand version from a right-hand animation:

```python
import bpy
import math
from mathutils import Euler

armature = bpy.data.objects['ARMATURE_NAME']
pose_bones = armature.pose.bones

def keyframe(bone_name, frame, rotation_degrees):
    bone = pose_bones[bone_name]
    bone.rotation_mode = 'XYZ'
    bone.rotation_euler = Euler((
        math.radians(rotation_degrees[0]),
        math.radians(rotation_degrees[1]),
        math.radians(rotation_degrees[2])
    ), 'XYZ')
    bone.keyframe_insert(data_path="rotation_euler", frame=frame)

# Define poses that can be mirrored
def apply_dribble_up_pose(side, frame):
    """Apply the 'hand up' pose to specified side ('L' or 'R')."""
    keyframe(f"hand.{side}", frame, (-30, 0, 0))
    keyframe(f"forearm.{side}", frame, (-10, 0, 0))
    for finger in ["f_index", "f_middle", "f_ring", "f_pinky"]:
        keyframe(f"{finger}.01.{side}", frame, (-15, 0, 0))
        keyframe(f"{finger}.02.{side}", frame, (-10, 0, 0))
        keyframe(f"{finger}.03.{side}", frame, (-10, 0, 0))

def apply_dribble_down_pose(side, frame):
    """Apply the 'hand down' pose to specified side ('L' or 'R')."""
    keyframe(f"hand.{side}", frame, (60, 0, 0))
    keyframe(f"forearm.{side}", frame, (25, 0, 0))
    for finger in ["f_index", "f_middle", "f_ring", "f_pinky"]:
        keyframe(f"{finger}.01.{side}", frame, (45, 0, 0))
        keyframe(f"{finger}.02.{side}", frame, (55, 0, 0))
        keyframe(f"{finger}.03.{side}", frame, (40, 0, 0))

# Create left hand dribble
apply_dribble_up_pose("L", 1)
apply_dribble_down_pose("L", 15)
apply_dribble_up_pose("L", 30)

print("Left hand animation created")
```

### Template: Frame View on Model

If the viewport isn't showing the model:

```python
import bpy

# Select the armature
bpy.ops.object.mode_set(mode='OBJECT')
bpy.ops.object.select_all(action='DESELECT')

armature = bpy.data.objects['ARMATURE_NAME']
armature.select_set(True)
bpy.context.view_layer.objects.active = armature

# Frame selected in viewport
for area in bpy.context.screen.areas:
    if area.type == 'VIEW_3D':
        override = {'area': area, 'region': area.regions[-1]}
        with bpy.context.temp_override(**override):
            bpy.ops.view3d.view_selected()
        break

# Return to pose mode
bpy.ops.object.mode_set(mode='POSE')
```

---

## Troubleshooting

### Common Errors and Solutions

#### "Could not connect to Blender"
- **Cause**: Blender MCP addon not running
- **Solution**: In Blender, enable the addon and start its server

#### "name 'math' is not defined"
- **Cause**: Forgot to import math module
- **Solution**: Add `import math` at the top of your code

#### "'Action' object has no attribute 'fcurves'"
- **Cause**: Blender API version difference
- **Solution**: Use alternative verification method (check bone rotations at frames instead)

#### Animation not visible
- **Cause**: Wrong armature selected, or viewing wrong area
- **Solution**: Use the "Frame View on Model" template to center the view

#### Keyframes not applying
- **Cause**: Not in Pose Mode, or wrong rotation mode
- **Solution**: Ensure `bpy.ops.object.mode_set(mode='POSE')` and `bone.rotation_mode = 'XYZ'`

### Debugging Tips

1. **Print everything**: Use `print()` liberally to confirm values
2. **Check frame by frame**: Loop through frames and print bone rotations
3. **Visual verification**: Always take screenshots at key frames
4. **Start simple**: Test with one bone before animating many

---

## Example: Complete Dribble Animation Workflow

Here's the complete workflow used to create a basketball dribble animation:

### 1. Explore Scene
```
mcp__blender__get_scene_info
→ Found: basicRig (ARMATURE), basicRig:Body (MESH)
```

### 2. Take Screenshot
```
mcp__blender__get_viewport_screenshot (max_size=800)
→ Visual: Arm model with visible bones
```

### 3. List Bones
```python
# execute_blender_code
import bpy
armature = bpy.data.objects['basicRig']
for bone in armature.pose.bones:
    print(bone.name)
# → Found 50 bones including hand.R, f_index.01.R, etc.
```

### 4. Create Animation
```python
# execute_blender_code - Full animation script (see templates above)
```

### 5. Verify Data
```python
# execute_blender_code
import bpy, math
armature = bpy.data.objects['basicRig']
hand = armature.pose.bones["hand.R"]
for frame in [1, 15, 30]:
    bpy.context.scene.frame_set(frame)
    print(f"Frame {frame}: {math.degrees(hand.rotation_euler.x):.1f}°")
# → Frame 1: -30.0°, Frame 15: 60.0°, Frame 30: -30.0° ✓
```

### 6. Visual Verification
```
Set frame to 1, take screenshot
Set frame to 15, take screenshot
Compare: Frame 1 shows open hand, Frame 15 shows cupped hand ✓
```

### 7. Declare Complete
Only after all verification passes.

---

## Summary Checklist for Animation Creation

1. [ ] Verify Blender MCP connection with `get_scene_info`
2. [ ] Take initial screenshot to understand scene
3. [ ] List all bones in armature
4. [ ] Plan animation keyframes (which bones, which frames, which rotations)
5. [ ] Clear existing animation data if needed
6. [ ] Create new Action with descriptive name
7. [ ] Set scene frame range
8. [ ] Create keyframes using helper function
9. [ ] Ensure loop points match (frame 1 = last frame)
10. [ ] Verify keyframe data by printing bone rotations at key frames
11. [ ] Take screenshots at frame 1 and middle frame for visual verification
12. [ ] Compare screenshots to confirm animation looks correct
13. [ ] Only then declare animation complete

---

## Reference Animation Values (Basketball Animations)

This section provides actual keyframe values from working basketball animations. Use these as starting points for natural-looking arm/hand movements.

### Understanding the Rig (IK-Based Arms)

This rig uses **Inverse Kinematics (IK)** for arm control:

| Bone | Control Method | Purpose |
|------|---------------|---------|
| `upper_arm.L/R` | Rotation (auto from IK) | Shoulder movement |
| `forearm.L/R` | Rotation (auto from IK) | Elbow bend |
| `forearm.L.001 / forearm.R.001` | **Location (IK Target)** | Controls where hand reaches |
| `hand.L/R` | Rotation | Wrist orientation |
| `f_*.01-03.L/R` | Rotation | Finger joints |
| `thumb.01-03.L/R` | Rotation | Thumb joints |

**Key Insight**: Move the IK target (`forearm.X.001`) to position the arm. The upper_arm and forearm rotations are computed automatically by the IK solver.

---

### Animation: Right Hand Dribble

**Duration**: 30 frames (1 second at 30fps)
**Motion**: Arm pushes down to dribble ball, returns up to catch

#### Frame 1 & 30 (Hand UP - Catching Ball)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.R.001 (IK) | - | (2.19, -0.37, 0.15) |
| hand.R | (20.1, -22.8, -73.2) | - |
| f_index.01.R | (-15, 0, 0) | - |
| f_middle.01.R | (-15, 0, 0) | - |
| thumb.01.R | (0, -20, 0) | - |

#### Frame 15 (Hand DOWN - Pushing Ball)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.R.001 (IK) | - | (2.43, -1.89, -0.86) |
| hand.R | (5.7, -6.0, -47.1) | - |
| f_index.01.R | (5, 0, 0) | - |
| f_middle.01.R | (5, 0, 0) | - |
| thumb.01.R | (0, 5, 0) | - |

#### IK Target Movement Summary

```
Frame 1 → 15: Move IK target DOWN and FORWARD
  Y: -0.37 → -1.89 (forward ~1.5 units)
  Z: 0.15 → -0.86 (down ~1.0 unit)
```

---

### Animation: Left Hand Dribble

**Duration**: 30 frames
**Motion**: Mirror of right hand dribble

#### Frame 1 & 30 (Hand UP)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.L.001 (IK) | - | (-2.19, -0.37, 0.15) |
| hand.L | (20.1, 22.8, 73.2) | - |
| f_index.01.L | (-15, 0, 0) | - |
| thumb.01.L | (0, 20, 0) | - |

#### Frame 15 (Hand DOWN)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.L.001 (IK) | - | (-2.43, -1.89, -0.86) |
| hand.L | (5.7, 6.0, 47.1) | - |
| f_index.01.L | (5, 0, 0) | - |
| thumb.01.L | (0, -5, 0) | - |

#### Mirroring Rules

When mirroring from Right to Left:
- **IK Location X**: Negate (flip sign)
- **IK Location Y, Z**: Keep same
- **Hand Rotation X**: Keep same
- **Hand Rotation Y, Z**: Negate (flip sign)
- **Finger Rotation X**: Keep same
- **Thumb Rotation Y**: Negate (flip sign)

---

### Animation: Basketball Shooting

**Duration**: 38 frames (~1.3 seconds at 30fps)
**Motion**: Wind-up → Lift → Release → Follow-through

#### Frame 1 (Starting Position - Ball Low)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.R.001 (IK) | - | (2.19, -0.37, 0.15) |
| hand.R | (20.1, -22.8, -73.2) | - |
| upper_arm.R | (-0.2, -0.3, 2.3) | - |

#### Frame 10 (Wind-up - Ball Rising)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.R.001 (IK) | - | (2.75, 0.55, 0.30) |
| hand.R | (-17.8, 4.8, -48.8) | - |
| upper_arm.R | (3.9, -11.9, -4.3) | - |

#### Frame 19 (Peak - Ball at Head Height)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.R.001 (IK) | - | (3.25, 2.67, 0.36) |
| hand.R | (-40.9, -4.3, 4.7) | - |
| upper_arm.R | (12.4, -9.1, -23.6) | - |
| forearm.R | (62.3, 17.2, -65.9) | - |

#### Frame 28 (Release/Follow-through - Arm Extended Up)

| Bone | Rotation (degrees) | Location |
|------|-------------------|----------|
| forearm.R.001 (IK) | - | (3.70, 6.08, 1.31) |
| hand.R | (84.3, -13.9, 80.0) | - |
| upper_arm.R | (94.2, 62.7, -4.2) | - |

#### Shooting Motion IK Path

```
Frame 1 → 28: IK target traces upward arc
  X: 2.19 → 3.70 (slight outward)
  Y: -0.37 → 6.08 (forward/up ~6.5 units - big movement!)
  Z: 0.15 → 1.31 (up ~1.2 units)
```

---

### Value Ranges Summary

#### IK Target Location Ranges (forearm.X.001)

| Motion | X Range | Y Range | Z Range |
|--------|---------|---------|---------|
| Dribble | ±2.2 to ±2.5 | -0.4 to -1.9 | 0.1 to -0.9 |
| Shooting | ±2.2 to ±3.7 | -0.4 to 6.1 | 0.1 to 1.3 |

#### Hand Rotation Ranges (hand.X)

| Axis | Dribble Range | Shooting Range | Purpose |
|------|--------------|----------------|---------|
| X | 5° to 20° | -41° to 84° | Wrist flex/extend |
| Y | -23° to -6° | -23° to -14° | Wrist twist |
| Z | -73° to -47° | -73° to 80° | Wrist tilt |

#### Finger Curl Ranges (f_*.01.X)

| Pose | X Rotation | Description |
|------|-----------|-------------|
| Spread/Open | -15° | Fingers extended, ready to catch |
| Neutral | 0° to 5° | Relaxed, slight curl |
| Cupped | 20° to 45° | Gripping ball |
| Tight Grip | 45° to 60° | Maximum curl |

---

### Tips for Natural Movement

1. **Don't over-curl fingers**: For dribbling, -15° (spread) to +5° (neutral) is enough
2. **IK target does the heavy lifting**: Move the IK target; let the solver handle arm angles
3. **Shooting is mostly Y-axis IK movement**: The big motion is forward/up (Y), not sideways
4. **Wrist follows hand position**: Adjust hand rotation after IK target is positioned
5. **Loop frames must match exactly**: Frame 1 = Frame 30 for seamless loops

---

## Exporting for Bevy

### Step 1: Protect Your Actions

Before exporting, ensure all actions have **Fake User** enabled so they're included:

```python
import bpy

for action in bpy.data.actions:
    action.use_fake_user = True
    print(f"Protected: {action.name}")
```

### Step 2: Export as glTF/GLB

1. **File → Export → glTF 2.0 (.glb/.gltf)**
2. In the export panel, ensure:
   - **Include → Data → Armatures** is checked
   - **Include → Data → Animations** is checked
   - **Animation → Export All Actions** is checked (or select specific ones)

### Step 3: Export Settings Recommendations

| Setting | Value | Why |
|---------|-------|-----|
| Format | GLB | Single binary file, easier to manage |
| Include Animations | ✓ | Obviously needed |
| Export All Actions | ✓ | Exports RightHandDribble, LeftHandDribble, etc. |
| Sampling Rate | 1 | Keyframe every frame for accuracy |
| Always Sample | ✓ | Ensures IK is baked to FK |

### Step 4: Using in Bevy

```rust
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the model with animations
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/player.glb#Scene0"),
        ..default()
    });
}

fn play_animation(
    mut players: Query<&mut AnimationPlayer>,
    animations: Res<Assets<AnimationClip>>,
) {
    for mut player in &mut players {
        // Play animation by name
        player.play(asset_server.load("models/player.glb#Animation0"));
        // Or by index - check the GLB to find which index is which action
    }
}
```

### Animation Names in Bevy

After export, animations are typically accessed by:
- `#Animation0`, `#Animation1`, etc. (by index)
- Or by name if using the `bevy_gltf_blueprints` or similar crate

To see which index corresponds to which action, you can inspect the GLB file or print the animation names when loading.

---

*Document Version: 2.0*
*Updated with reference animation values from basketball dribble and shooting animations*
