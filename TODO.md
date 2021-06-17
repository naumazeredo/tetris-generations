# Tasks

## v0.1

### Game

- [x] Playfield
  - [x] Height 40 (starts from bottom)
- [ ] Rules
  - [x] Refactor rules folder and create methods for each rule (soft_drop, hard_drop, clear_lines,
      etc)
  - [ ] Refactor SinglePlayerScene into RulesInstance (maybe a better name)
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
    - [x] Hold
      - [x] Reset rotation
    - [x] Ghost piece
    - [ ] Spawn drop
    - [x] Spawn row
    - [ ] ARE
    - [ ] Lock delay
    - [ ] Preview pieces
      - [x] Next piece
      - [ ] 2+ pieces layout
    - [ ] Extended orientations
  - [x] Randomizer
    - [x] Full random
    - [x] Random Generator (7-Bag)
  - [ ] Top out
    - [ ] Block out
    - [ ] Lock out
- [ ] Input mapping from guideline
  - [ ] Controller
    - [x] Normal buttons
    - [ ] Triggers
  - [x] Keyboard
- [x] Scene system
- [x] Animations
  - [x] Piece movement
  - [x] Line clear
    - [x] Classic
- [x] Basic piece colors
- [x] [cleanup] Move piece draw functions to game/piece.rs and reuse them in debug_pieces scene
- [x] Restart button for testing
- [ ] Playfield grid (1px)

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
    - [x] Key repeat
  - [x] Feedback
    - [x] Rumble normal
- [x] [render]
  - [x] Font rendering
  - [x] Improve rendering performance
- [ ] Asset system
- [ ] Logger system
- [x] Refactor systems to match
- [x] ImDraw derive to enums
- [x] Split debug system from the rest (to be able to use dear imgui to draw the app)
  - (Ended up refactoring the App new and run to be able to do this consistently)

#### Issues

- [ ] [bug] Controller buttons are not working for some
- [ ] [bug] Piece movement animation makes the rendering be outside of the playfield. This should be fixed
    with rendering on a framebuffer instead of directly on the screen
- [x] [system design] Rename animations, time and tasks to *_system
- [x] [deps] rust-sdl2 subsystems should be copied instead of referenced. We may refactor a lot of the app
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
- [ ] Improve animations
- [ ] Game menu
  - [ ] Select game rules and start game
- [ ] Scoring
- [ ] Speed/difficulty progression
- [ ] Draw grid option
- [ ] 9-slicing texture for windows
- [ ] Fix DAS -> DAS (Delayed Auto Shift) is verified only when the time has passed: it only stops
    the repeating movement if, when verified the next step, the key is not pressed.
    - [x] DAS charge
- [ ] Customizable piece styles

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
  - [ ] Stack commands (change matrices, color alpha, etc)
  - [ ] Batch rendering
  - [ ] Shader struct
  - [ ] Render to framebuffer + post render effects
  - [ ] verify gl errors
- [ ] Test all parts
- [ ] [entities] gen_containers: add len for entity type
- [ ] [imdraw] Remove imgui::im_str2
- [ ] [imdraw derive] check for #[imdraw_ignore] to not show some fields

### Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image
