# Tasks

## Future

- we need tests!
- matchmaking, please?
- all rules, finally
- time to refactor time
- editor stuff
- the real input support

## v0.0.2: we have a game (roughly)

### Game

- [ ] General refactor/cleanup
  - [x] Refactor SinglePlayerScene into RulesInstance (maybe a better name)
  - [ ] Refactor orientation rule usage and rules functions
  - [ ] [game render] Change all rendering to be pixel perfect
    - [x] pixel_scale to u8
    - [ ] BLOCK_SCALE to u8
    - [ ] All render functions to receive Vec2i or integers instead of floats
- [ ] Scenes
  - [x] Main menu
    - [ ] Background
    - [ ] Options
      - [ ] Video
      - [ ] Audio
      - [ ] Controls
  - [ ] Classic game
    - [ ] Local
    - [ ] Multiplayer
  - [ ] Modern game
    - [ ] Local
    - [ ] Multiplayer
  - [ ] Custom game
- [ ] [render] Clip render of blocks to playfield
  - [ ] ~[bug] Piece movement animation makes the rendering be outside of the playfield. This should be
      fixed with rendering on a framebuffer instead of directly on the screen~
- [ ] [bug] multiplayer scene -> game over = crash (after some time)?
- [ ] [bug] multiplayer spectate scene -> connecting is not dropping after server died?
- [ ] [visual bug] animations are skipped if the piece is locked in the middle of the animation
- [ ] [bug] Controller buttons are not working for some reason (working fine for now, but no changes
    were made, so this bug is still there)

### Engine

- [ ] Refactor systems to be data and App interface to implement the logic
  - [ ] Refactor systems to have a uniform interface (system/mod.rs should contain the pub
      interface)
- [ ] [network]
  - [x] Rename app/net to app/network
  - [x] public interface instead of whole struct being public
  - [ ] Server-client
    - [ ] Connection
      - [x] Naive
      - [ ] Challenge (remove connection IP spoofing)
    - [x] Heartbeat
      - [x] Request/reply
  - [ ] Serialization
    - [ ] Bit packing
      - [ ] Integer
        - [x] i8, i16, i32, i64, u8, u16, u32, u64
        - [ ] i128, u128
      - [ ] Floating point
- [ ] [logging]: just improve logging.
- [ ] [video]
  - [x] Get display modes
  - [x] use sdl2::video::{DisplayMode, FullscreenType} in video_system since we use the structs
  - [ ] [issue] Change screen mode to desktop doesn't change display mode (going back to fullscreen
      won't be on monitor resolution)
  - [ ] [issue] Change screen mode to fullscreen and back to windowed should restore windowed size
- [x] [audio]
  - [x] Music loading
  - [x] Sfx loading
  - [x] Volume mixer
    - [x] Music + SFX (all channels)
- [x] [render]
  - [x] Axis-aligned (+ scissor) rendering
  - [x] Stack of modifier commands
    - [x] Scissor/Clip
- [ ] [ui]
  - [x] Render scissor
  - [ ] Widgets
    - [x] Text
      - [x] Basic functionality
      - [x] State+Build pattern
      - [ ] Allow more sizes?
      - [ ] Centered
      - [ ] Header (maybe just centered text and custom text size?)
    - [x] Button
      - [x] Basic functionality
      - [x] State+Build pattern
    - [x] Checkbox
      - [x] Basic functionality
      - [x] State+Build pattern
    - [ ] Input integer range
      - [x] Basic functionality
      - [ ] State+Build pattern
    - [x] Input text
      - [x] Basic functionality
      - [x] State+Build pattern
    - [ ] Combobox
      - [x] Basic functionality
      - [x] State+Build pattern (changed, changing?)
      - [ ] Left/right control (seems to be the simplest and better control)
    - [ ] Slider
      - [x] Basic functionality
      - [x] Disabled colors
      - [ ] State+Build pattern
        - [ ] Annoying to deal with multiple integer types
    - [ ] Input float range
    - [ ] Input color
    - [ ] Input key
  - [x] Disabled widgets
  - [ ] Keyboard/Controller support
    - [ ] Selected line
    - [ ] Styling colors for text/widgets (or colored background of the line)
- [x] [input]
  - [x] Use real time and somehow manage game system

## v0.0.1: The start of the journey

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
- [x] Scoring
  - [x] Piece drops
    - [x] Soft drop
    - [x] Hard drop
  - [x] Single/Double/Triple/Tetris
- [x] Speed/difficulty progression
  - [x] Somehow treat both starting at level 0 or 1 (some games start at level 0, some others start
      at level 1)

### Engine

- [x] [input]
  - [x] Mapping
    - [x] Basic mapping
  - [x] Virtual button
    - [x] Keyboard
    - [x] Mouse
    - [x] Controller button
    - [x] Controller axis
    - [x] Key repeat
  - [x] Feedback
    - [x] Rumble
- [x] [render]
  - [x] Font rendering
  - [x] Improve rendering performance
- [x] Refactor systems to match
- [x] [debug]
  - [x] Create imgui macro to draw structs
  - [x] Split debug system from the rest (to be able to use dear imgui to draw the app)
      (Ended up refactoring the App new and run to be able to do this consistently)
  - [x] [imdraw] ImDraw derive to enums
  - [x] [imdraw] Enums should have the variant name in label
- [x] [video] Fix fullscreen mode
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
- [ ] Scoring
  - [ ] Hurdle/Split (https://tetris.fandom.com/wiki/Line_clear)
  - [ ] Back-to-back (https://tetris.fandom.com/wiki/Line_clear)
  - [ ] Combo (https://tetris.fandom.com/wiki/Line_clear)
  - [ ] Twists/Spins (https://tetris.fandom.com/wiki/List_of_twists)
- [ ] Improve animations
- [ ] Game menu
  - [ ] Select game rules and start game
  - [ ] Styling options
    - [ ] Draw grid option
    - [ ] Customize piece styles (maybe be able to add new styles)
    - [ ] [game render] Draw functions with different block textures and different pixel scales
- [ ] 9-slicing texture for windows
- [ ] Fix DAS/ARR -> DAS (Delayed Auto Shift) is verified only when the time has passed: it only stops
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

- [ ] Address cargo clippy warnings
- [ ] Memory allocation
  - [ ] Have a custom allocator to remove most use cases of dynamic arrays (Vec)
  - [ ] Frame allocator
- [ ] Test systems
- [ ] [animation system / task system] task system turned out to be a bad idea. We may need to find
    a better, more reliable solution to it.
    - Maybe a stackable task system could be a good solution to be able to add it to the scenes (and
      update with dt instead of game time?)
    - Doing this can be a good opportunity to remove the State generics from App!
- [ ] [render]
  - [ ] [bug] [font render] Fix background not being transparent -> rendering issue: z orders not
      matching rendering order. Maybe just order everything by the z order (more draw calls might be
      needed and the state function changes should be stored in the post processed draw call
      somehow)
  - [ ] Stack commands
    - [ ] Blending
    - [ ] Matrix transformation
  - [ ] Batch rendering
  - [ ] Shader struct
  - [ ] Render to framebuffer + post render effects
  - [ ] Check GL errors
  - [ ] More backend supports (Vulkan, DirectX)
  - [ ] Multithreading
    - [ ] Maybe use persistent structures for rendering (or have a double/triple buffer)
- [ ] [input]
  - [ ] [issue] Not updating an input_mapping can break it: maybe having a local timer for each
      input_manager, getting the whole global key state on enabling and calling update with dt will
      solve all related problems.
    - [ ] [issue] It may also be related to: how to address 'pressed_for' and opening
        an UI (opening the UI will probably disable the input_mapping, since it moved the focus, and
        the 'pressed_for' will be using real time)
  - [ ] Buffer input for some frames: good when character is hit and the player starts hitting
      buttons before the character is completely functional
    - [ ] Input sequences: good to grab complicated sequences or 2+ buttons (Circle+Triangle) as a
        single action
  - [ ] Flush/Reset (what is this? I forgot)
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
- [ ] [audio]
  - [ ] Fine tuning (AppConfig)
  - [ ] Volume mixer
    - [ ] Individual channel control
- [ ] [video]
  - [ ] Abstract FullscreenType since it has a quite bad name for all variants
- [ ] [network]
  - [ ] Packet fragmentation
  - [ ] Server-client
    - [ ] Connection
      - [ ] Reliable UDP
      - [ ] Encryption
    - [ ] Heartbeat
      - [ ] Timeout state
    - [ ] Server slots (max number of players)
    - [ ] Server configuration
      - [ ] Retry duration
      - [ ] Timeout duration
  - [ ] Serialization
    - [ ] Bit packing
      - [ ] Strings (?)
    - [ ] Reserve bits
    - [ ] Derive macro
  - [ ] Twitch integration
  - [ ] Steam integration
  - [ ] Matchmaking
- [ ] [assets]
  - [ ] Asset System should not hold all data, each component can hold its own data.
      It should handle what should be loaded and unloaded into/from memory (seems that this has is
      somewhat what a Scene Manager is for, though, except for streaming data)
- [ ] [time]
  - [ ] Make game run in frames instead of continuous time
  - [ ] Timer data structure (and dt in update functions)
      - (always using game time makes it really hard to stack. Moving from one scene to another, game
      over action, etc)
      - (design: `app.update_timer(&mut timer); let elapsed_time = timer.elapsed_time();`)
  - [ ] Duration/Time struct (mainly to better integrate with SDL times): maybe use
      Instant/Duration?
- [ ] [ui]
  - [ ] Custom layout
    - [ ] Stretching sizes
    - [ ] Anchored positioning
  - [ ] Cache rendered components and windows
  - [ ] Maybe have a "multiline" window (header + multiple lines) and a "freestyle" (hand placed
      widgets): not sure how other UIs will work yet (HUD? maybe just more widgets, trying to not
      generalize)
  - [ ] UI stack? This will be needed if we want a Monster Hunter way of doing UI (controlling the
      character works fine when UI is opened, but only the top of the UI stack accepts UI input)
  - [ ] Position/size should be calculated on rendering call. The calls should store the layout and
      state (this will make the rendering delayed by 1 frame, which should might fine)
    - [ ] Layout: size (auto+min+max, fixed)
    - [ ] Extra commands: indent, unindent, same line, (group start/end? -> maybe this will require
        two passes: calculate position/size, render)
    - [ ] Animations
  - [ ] Widgets
    - [ ] Combobox
      - [ ] Combowheel?
      - [ ] Enum macro?
    - [ ] Separator
  - [ ] Keyboard/Controller support
    - [ ] Styling colors for text/widgets (or colored background of the line)
  - [ ] Styling options
  - [ ] Custom shader
  - [ ] Multipage window ("multiline" window)
  - [ ] Align to bottom ("multiline" window)
  - [ ] Update state on string changes: we are not comparing string changes for most widgets, and we
      only update state in case we see the difference. This should be done with a string hashing
      system to avoid copying the strings over and over
  - [ ] Label as format string? (maybe enable a flag in builder: .format_str())
  - [ ] [optimization] Bake style into a better struct to avoid branching on rendering
- [ ] [editor]
  - [ ] Remove Dear ImGUI
    - [ ] [imdraw]
      - [ ] Remove imgui::im_str2
      - [ ] derive: check for #[imdraw_ignore] to not show some fields
  - [ ] Implement basic Editor functionality using UI System
    - [ ] Save/load UI designs
- [ ] [scene]: Refactor into App (how to do this properly without vtables? I would like to have the
    App being in control of the scene manager while the game will give the scenes)

### Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image
