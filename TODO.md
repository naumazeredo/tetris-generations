# Tasks

## v0.1

### Game

- [x] Playfield
  - [x] Height 40 (starts from bottom)
- [ ] Rules
  - [x] Refactor rules folder and create methods for each rule (soft_drop, hard_drop, clear_lines,
      etc)
  - [ ] Refactor SinglePlayerScene into RulesInstance (maybe a better name) and the scene handling
  - [x] Line clear
    - [x] Naive
  - [ ] Rotation systems
    - [x] Original
    - [ ] SRS
      - [ ] Wall kick SRS
  - [ ] General Mechanics
    - [x] Drop
      - [x] Gravity
    - [x] Soft drop
    - [x] Hard drop
    - [ ] Hold
      - [ ] Reset rotation
    - [ ] Ghost piece
    - [ ] Spawn drop
    - [ ] Spawn row
    - [ ] ARE
    - [ ] Lock delay
    - [ ] Preview
    - [ ] Extended orientations
  - [x] Randomizer
    - [x] Full random
    - [x] Random Generator (7-Bag)
  - [ ] Top out
    - [ ] Block out
    - [ ] Lock out
- [ ] Input mapping from guideline
- [ ] DAS
- [ ] Scene system

### Engine

- [x] Create imgui macro to draw structs
- [x] Input system
  - [x] Mapping
    - [x] Basic mapping
  - [x] Virtual button
    - [x] Keyboard
    - [x] Mouse
    - [x] Controller button
    - [x] Controller axis
    - [ ] Key repeat
  - [x] Feedback
    - [x] Rumble normal
- [x] [render]
  - [x] Font rendering
  - [x] Improve rendering performance
- [ ] Asset system
- [ ] Logger system
- [x] Refactor systems to match
- [ ] ImDraw derive to enums

#### Issues

- [x] Rename animations, time and tasks to *_system
- [x] rust-sdl2 subsystems should be copied instead of referenced. We may refactor a lot of the app
    code

## Backlog

### Game

- [ ] Rules
  - [ ] Line clear
    - [ ] Sticky
    - [ ] ?
  - [ ] Rotation systems
    - [ ] Nintendo - Left Handed
    - [ ] Nintendo - Right Handed
    - [ ] Sega
    - [ ] ARS
    - [ ] DTET
  - [ ] General Mechanics
    - [ ] Soft drop speed
    - [ ] Firm drop
    - [ ] IRS
    - [ ] IHS
    - [ ] General piece positioning
      - [ ] Round left
      - [ ] Right handed
      - [ ] Flat side up/down
    - [ ] Wall kick rules
      - [ ] Original
      - [ ] TGM
      - [ ] TGM3
      - [ ] DX
      - [ ] DTET
  - [ ] Randomizer
    - [ ] TGMACE
    - [ ] TGM1
    - [ ] TGM
    - [ ] TGM3
  - [ ] Top out
    - [ ] Partial lock out
    - [ ] Garbage out

### Engine

- [ ] Input system
  - [ ] Flush/Reset
  - [ ] Mapping
    - [ ] Bind mapping to a controller and detect input change from keyboard to controller
  - [ ] Virtual button
    - [ ] Joystick button
    - [ ] Joystick axis
    - [ ] (extra) Multimedia button
  - [ ] Virtual axis
    - [ ] Keyboard
    - [ ] Mouse
    - [ ] Controller button
    - [ ] Controller axis
    - [ ] Joystick button
    - [ ] Joystick axis
    - [ ] (extra) Multimedia button
  - [ ] Feedback
    - [ ] Dualsense extra feedbacks
- [ ] [ui] Command buffer "immediate mode"
- [ ] [debug] imgui architecture make it not possible to pass App down to callbacks
- [ ] [debug] rename to Editor and implement an Immediate Mode GUI from scratch (or use the, to be
    implemented, UI system)
- [ ] [render]
  - [ ] Batch rendering
  - [ ] Shader struct
  - [ ] Render to framebuffer + post render effects
  - [ ] verify gl errors
- [ ] Test all parts
- [ ] [entities] gen_containers: add len for entity type
- [ ] [imdraw derive] implement for enums

### Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image
