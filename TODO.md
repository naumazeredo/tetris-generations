# Tasks

## v0.1

### Game

- [x] Playfield
  - [x] Height 40 (starts from bottom)
- [x] Rules
  - [x] Refactor rules folder and create methods for each rule (soft_drop, hard_drop, clear_lines,
      etc)
  - [x] Line clear
    - [x] Naive
  - [ ] Rotation systems
    - [x] Original
    - [x] SRS
      - [x] Wall kicks
    - [x] Nintendo - Left Handed
    - [x] Nintendo - Right Handed
    - [x] Sega
  - [x] General Mechanics
    - [x] Drop
      - [x] Gravity
    - [x] Soft drop
    - [x] Hard drop
    - [x] Hold
      - [x] Reset rotation
    - [x] Ghost piece
    - [x] Spawn drop
    - [x] Spawn row
    - [x] ARE
    - [x] Lock delay
      - [x] Entry reset
      - [x] Step reset
      - [x] Move reset
    - [x] Preview pieces
      - [x] Next piece
      - [x] 2+ pieces layout
  - [x] Randomizer
    - [x] Full random
    - [x] Random Generator (7-Bag)
  - [x] Top out
    - [x] Block out
    - [x] Lock out
    - [x] Partial lock out
  - [x] Rotation system orientations
    - [x] Original
    - [x] NRSR
    - [x] NRSL
    - [x] Sega
    - [x] ARS
    - [x] SRS
    - [x] DTET
- [x] Input mapping from guideline
  - [x] Controller
    - [x] Normal buttons
    - [x] Triggers
  - [x] Keyboard
  - [x] Pause
- [x] Scene system
- [x] Animations
  - [x] Piece movement
  - [x] Line clear
    - [x] Classic
- [x] Basic piece colors
- [x] [cleanup] Move piece draw functions to game/piece.rs and reuse them in debug_pieces scene
- [x] Restart button for testing
- [x] Playfield grid (1px)
- [ ] Scoring
  - [x] Piece drops
    - [x] Soft drop
    - [x] Hard drop
  - [x] Single/Double/Triple/Tetris
  - [ ] Hurdle/Split (https://tetris.fandom.com/wiki/Line_clear)
  - [ ] Back-to-back (https://tetris.fandom.com/wiki/Line_clear)
  - [ ] Combo (https://tetris.fandom.com/wiki/Line_clear)
  - [ ] Twists/Spins (https://tetris.fandom.com/wiki/List_of_twists)
- [x] Speed/difficulty progression
  - [x] Somehow treat both starting at level 0 or 1 (some games start at level 0, some others start
      at level 1)

- [ ] General refactor/cleanup
  - [x] Refactor SinglePlayerScene into RulesInstance (maybe a better name)
  - [ ] Refactor orientation rule usage and rules functions
  - [ ] [game render] Change all rendering to be pixel perfect
    - [x] pixel_scale to u8
    - [ ] BLOCK_SCALE to u8
    - [ ] All render functions to receive Vec2i or integers instead of floats

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
- [ ] Timer data structure (and dt in update functions)
    - (always using game time makes it really hard to stack. Moving from one scene to another, game
    over action, etc)
    - (design: `app.update_timer(&mut timer); let elapsed_time = timer.elapsed_time();`)
- [x] Refactor systems to match
- [x] ImDraw derive to enums
- [x] Split debug system from the rest (to be able to use dear imgui to draw the app)
  - (Ended up refactoring the App new and run to be able to do this consistently)
- [x] [imdraw] Enums should have the variant name in label
- [x] Fix fullscreen mode
- [ ] Duration/Time struct (mainly to better integrate with SDL times)

#### Issues

- [ ] [game animations] animations are skipped if the piece is locked in the middle of the animation
- [ ] [bug] Controller buttons are not working for some reason (working fine for now, but no changes
    were made, so this bug is still there)
- [ ] [bug] Piece movement animation makes the rendering be outside of the playfield. This should be
    fixed with rendering on a framebuffer instead of directly on the screen
- [x] [system design] Rename animations, time and tasks to *_system
- [x] [deps] rust-sdl2 subsystems should be copied instead of referenced. We may refactor a lot of
    the app code

## Backlog

### Game

- [ ] Rules instance
  - [ ] Split render functions to each component: the scene should be responsible for the rendering
      positions and what should be rendered (playfield pos, windows positions, etc, should be passed
      to the rendner functions, not stored into RulesInstance)
- [ ] Rules
  - [ ] Line clear
    - [ ] Sticky
    - [ ] ?
  - [ ] Rotation systems
    - [ ] ARS
    - [ ] DTET
  - [ ] General Mechanics
    - [ ] Lock delay
      - [ ] Move reset Infinity
      - [ ] Step reset not infinity?
    - [ ] Soft drop speed
    - [ ] Firm drop
    - [ ] IRS
    - [ ] IHS
    - [ ] Wall kick rules
      - [ ] TGM
      - [ ] TGM3
      - [ ] DX
      - [ ] DTET
  - [ ] Top out
    - [ ] Garbage out
  - [ ] Randomizer
    - [ ] TGMACE
    - [ ] TGM1
    - [ ] TGM
    - [ ] TGM3
- [ ] Improve animations
- [ ] Game menu
  - [ ] Select game rules and start game
  - [ ] Styling options
    - [ ] Draw grid option
    - [ ] Customize piece styles (maybe be able to add new styles)
    - [ ] [game render] Draw functions with different block textures and different pixel scales
- [ ] 9-slicing texture for windows
- [ ] Fix DAS -> DAS (Delayed Auto Shift) is verified only when the time has passed: it only stops
    the repeating movement if, when verified the next step, the key is not pressed.
    - [x] DAS charge
- [ ] Rotation system tutorial/explanations
  - Show which rules and timings the system has
  - Show lock delay animation (show a progress bar indicating the duration, show when it keeps going
      and when it resets)
  - Show all wall kicks (with animations: show piece moving, when rotation was triggered and which
      positions it tested against)
  - Show all piece orientations (with spawn height indicator)
- [ ] [test] Test all wall/floor kick rotations!
- [ ] PAL vs NTSC framerates (update speeds to frames)

### Engine

- [ ] Input system
  - [ ] Flush/Reset
  - [ ] Mapping
    - [ ] Bind mapping to a controller and detect input change from keyboard to controller
  - [ ] Virtual button
    - [ ] Accumulated time (instead of taking the time difference)
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
- [ ] [font render] fix background not being transparent
- [ ] [animation system / task system] task system turned out to be a bad idea. We may need to find
    a better, more reliable solution to it.
    - Maybe a stackable task system could be a good solution to be able to add it to the scenes (and
      update with dt instead of game time?)
    - Doing this can be a good opportunity to remove the State generics from App!
- [ ] Multithread
  - [ ] Persistent structures for rendering

### Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image
