# Tasks

## Future

- we need tests!
- matchmaking, please?
- all rules, finally
- time to refactor time
- editor stuff
- the real input support
- adding audio

## v0.0.2: we have a game (roughly)

### Game

- [ ] Fix Classic style: NES Classic has blocks with sizes 7x7, not 8x8
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
      - [ ] Select level menu
      - [ ] Change layout to match NES
      - [ ] Options in pause menu
    - [ ] Multiplayer
  - [ ] Modern game
    - [ ] Local
    - [ ] Multiplayer
  - [ ] Custom game
    - [ ] Explaination for each rule
      - [ ] Times should show a realtime + slowdown near the action (or step by step explaination)
      - [ ] Rotation systems should show all pieces rotations and have multiple pages showing the
          kicks
- [x] [render] Clip render of blocks to playfield
  - [x] ~[bug] Piece movement animation makes the rendering be outside of the playfield. This should be
      fixed with rendering on a framebuffer instead of directly on the screen~
- [ ] [bug] multiplayer scene -> game over = crash (after some time and after a failed connect)?
- [ ] [bug] multiplayer spectate scene -> connecting is not dropping after server died?
- [ ] [visual bug] animations are skipped if the piece is locked in the middle of the animation
- [ ] [bug] Controller buttons are not working for some reason (working fine for now, but no changes
    were made, so this bug is still there)
- [ ] [scene manager]: Refactor into App (how to do this properly without vtables? I would like to have the
    App being in control of the scene manager while the game will give the scenes)
  - [ ] Create a derive macro like enum_dispatch (can't use enum dispatch with associated type)
  - [ ] Use associate type in SceneTrait to be able to move SceneManager to App
  - [ ] Add generic type Scene to App to move SceneManager to App

### Engine

- [ ] Refactor projects: lib (engine), bins (game, server, etc)
    linalg should be moved to the app
- [ ] Refactor systems to be data and App interface to implement the logic
  - [ ] Refactor systems to have a uniform interface (system/mod.rs should contain the pub
      interface) and avoid App self borrow:
      app.time.delta(), app.tasks.queue_task(..), app.renderer.queue_draw(..)
  - [ ] Refactor App into multiple traits to be able to have mockable parts to test everything
      app.time().delta(), app.tasks().queue_task(..), app.renderer().queue_draw(..)
      - This may be annoying since it would add a mut self reference to app
- [x] [network]
  - [x] Rename app/net to app/network
  - [x] public interface instead of whole struct being public
  - [x] Server-client
    - [x] Connection
      - [x] Naive
    - [x] Heartbeat
      - [x] Request/reply
  - [x] Serialization
    - [x] Bit packing
      - [x] Integer
        - [x] i8, i16, i32, i64, u8, u16, u32, u64
- [ ] [logging]: just improve logging.
- [ ] [video]
  - [x] Get display modes
  - [x] use sdl2::video::{DisplayMode, FullscreenType} in video_system since we use the structs
  - [x] Add vsync option
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
  - [x] Batch rendering
  - [x] Render to framebuffer
  - [x] Subtexture structure: currently we only have Sprite, but a sprite is subtexture with more
      info
  - [ ] Text rendering
    - [ ] Multiline line spacing
    - [ ] Multiline \n
    - [ ] Multiline break on strings
    - [ ] Escape commands (change color, change font size?, add texture?)
- [ ] [ui]
  - [x] Remove header? it doesn't make it better (doesn't deal with multiline texts or anything
      special). Just adding a Text widget is good enough
  - [x] Render scissor
  - [x] Multipage window ("multiline" window)
      - Maybe scroll page? Should need to split header and footer elements to not scroll with the
          page
  - [ ] Line focus?
  - [ ] Widgets
    - [ ] Text
      - [x] Basic functionality
      - [x] State+Build pattern
      - [x] Header (maybe just centered text and custom text size?)
      - [ ] Multiline
        - [x] Multiline text widget
        - [ ] Line spacing
        - [ ] Alignment options
    - [x] Button
      - [x] Basic functionality
      - [x] State+Build pattern
    - [x] Checkbox
      - [x] Basic functionality
      - [x] State+Build pattern
    - [ ] Input integer range
      - [x] Basic functionality
      - [ ] State+Build pattern
      - [ ] Scroll interaction
    - [x] Input text
      - [x] Basic functionality
      - [x] State+Build pattern
    - [ ] Combobox
      - [x] Basic functionality
      - [x] State+Build pattern (changed, changing?)
      - [x] Center text to match slider
      - [ ] Enum
        - [ ] Macro to generate list of strings from enum
        - [ ] Enum as generics (get list of strings as trait and receive the value as enum type
            directly)
    - [ ] Slider
      - [x] Basic functionality
      - [x] Disabled colors
      - [x] State+Build pattern
      - [ ] Value step
      - [ ] Scroll interaction
    - [ ] Paged Box
      - [x] Basic functionality
      - [x] State+Build pattern
      - [ ] Click arrow to change pages
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
- [ ] Correct NES resolutions and scaling (PAL: 256x240, NTSC: 256x224 with bottom 8 scanlines not
    visible)
- [ ] Easter eggs
  - [ ] Main menu match
    - [ ] Have a Tetris game (classic) in the main menu. After a Tetris, the menu disappears, the
        playfield centers, the classic music starts playing. Then the game goes on each generation
        of Tetris, changing the rules, rotation, music, sounds, effects, graphics, etc.
  - [ ] Nintendo code?
- [ ] Tetris Tutor
  - [ ] Tutorials explaining the mechanics, strategies and META for Classic and Modern (maybe
      variations: no rotation, time attack, 40 lines, perfect clears, battle)

### Engine

- [ ] Address cargo clippy warnings
- [ ] Remove usize variables (why would I need 8 bytes for most things?)
- [ ] Test systems
- [ ] Document everything
- [ ] Memory allocation
  - [ ] Have a custom allocator to remove most use cases of dynamic arrays (Vec)
  - [ ] Frame allocator
- [ ] [animation system / task system] task system turned out to be a bad idea. We may need to find
    a better, more reliable solution to it.
    - Maybe a stackable task system could be a good solution to be able to add it to the scenes (and
      update with dt instead of game time?)
    - Doing this can be a good opportunity to remove the State generics from App!
- [ ] [render]
  - [ ] [bug] Fix background not being transparent -> rendering issue: z orders not
      matching rendering order. Maybe just order everything by the z order (more draw calls might be
      needed and the state function changes should be stored in the post processed draw call
      somehow)
  - [ ] [fix] Framebuffer rendered textures have inverted y. Maybe we should fix by flipping the
      viewport (or maybe it's just broken, we have to test with subtextures not containing the whole
      height!). queue_draw_texture should also accept flip as a parameter to give full control.
  - [ ] Have a way to delete allocated textures: this depends on AssetManager design decisions
  - [ ] Pixel perfect rendering
  - [ ] Batch rendering
    - [ ] Stack commands
      - [ ] Blending
      - [ ] Matrix transformation
  - [ ] Shader struct
    - [ ] Store all uniforms (glGetProgramiv) (do we need to store the attributes also?)
    - [ ] Set attribute values during execution
  - [ ] Check GL errors
  - [ ] More backend supports (Vulkan, DirectX)
  - [ ] Multithreading
    - [ ] Maybe use persistent structures for rendering (or have a double/triple buffer)
  - [ ] [video?] Have a way to resize nicely: resizing the window doesn't change the rendering.
      Maybe we should have specific layouts for specific aspect-ratios and make the renderer
      re-scale based on the resolution
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
  - [ ] Clipboard support
- [ ] [audio]
  - [ ] Fine tuning (AppConfig)
  - [ ] Volume mixer
    - [ ] Individual channel control
- [ ] [video]
  - [ ] Abstract FullscreenType since it has a quite bad name for all variants
  - [ ] Abstract SwapInterval
  - [ ] [render?] Have a way to resize nicely: resizing the window doesn't change the rendering.
      Maybe we should have specific layouts for specific aspect-ratios and make the renderer
      re-scale based on the resolution
- [ ] [network]
  - [ ] Packet fragmentation
  - [ ] Server-client
    - [ ] Connection
      - [ ] Reliable UDP
      - [ ] Encryption
      - [ ] Challenge (remove connection IP spoofing)
    - [ ] Heartbeat
      - [ ] Timeout state
    - [ ] Server slots (max number of players)
    - [ ] Server configuration
      - [ ] Retry duration
      - [ ] Timeout duration
  - [ ] Serialization
    - [ ] Bit packing
      - [ ] Integer
        - [ ] i128, u128
      - [ ] Floating point
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
  - [ ] Asset system config file: there should be some files listing all assets being used, the name
      of the asset, what's the actual file this asset is at.
    - For textures it should know the rect of the image the asset represents
    - For sounds it should know the interval of the music file the asset represents
    - (There's probably more types that might be packed)
    - [ ] Pack textures: to not manually need to pack them, we should be able to 
    - [ ] Versioning: for packaging, we shouldn't modify already packaged config files (or packed
        textures). So there might be multiple asset config files describing the assets (this seems
        similar to virtual filesystem part also)
  - [ ] Hold sprites?
  - [ ] get_ui_sprite: Change sprite depending on most recent input type (keyboard/mouse, Xbox
      controller, PS controller, etc)
- [ ] [font]
  - [ ] Change from sdl-ttf to ttf-parser (https://github.com/RazrFalcon/ttf-parser) + render.
      It's not that easy to just change to stb-truetype since translating a C lib is not so direct
      and the crate that does this is not maintained anymore).
  - [ ] Store multiple font sizes or a chosen value that has a lot of divisors, to avoid conversion
      issues
  - [ ] [render] Make a builder for calculate_draw_size
  - [ ] [render] Fix text rendering at y = -text_size instead of y = 0
- [ ] [time]
  - [ ] Make game run in frames instead of continuous time
  - [ ] Timer data structure (and dt in update functions)
      - (always using game time makes it really hard to stack. Moving from one scene to another, game
      over action, etc)
      - (design: `app.update_timer(&mut timer); let elapsed_time = timer.elapsed_time();`)
  - [ ] Duration/Time struct (mainly to better integrate with SDL times): maybe use
      Instant/Duration?
- [ ] [ui]
  - [ ] UI code seems quite bad on system side: placers seems like a good idea but it turned out to
      have a pretty bad code. We might need to redo the whole UI code design later
  - [ ] Not doing: ~["multiline" window] Maybe refactor to be "widget centric" and left/right parts
      - Right now each widget can have a left and right elements (not forced in the design, was just
          coincidental) and the elements don't even know which widget they are.
      - Maybe we should degeneralize (specify?) and force to be left+right or full line widget
      - Widgets should know their full sizes to allow line focus and to have a better keyboard
          support~
  - [ ] Resolution/aspect ratios configurations: changing resolutions might require changing the UI
      layout also. We should have preconfigured layouts for common aspect ratios and resolutions and
      approximate in case it's not matching
      - We need resolution (or height, or width) + aspect ratio since the same aspect ratio with a
          smaller resolution can easily make the UI look quite bad (too small).
  - [ ] UI scaling
  - [ ] Custom layout
    - [ ] Stretching sizes
    - [ ] Anchored positioning
      - [ ] ~Align to bottom: footer? maybe explicitly add a header also? ("multiline" window)~
  - [ ] Cache rendered components and windows
  - [ ] Maybe have a "multiline" window (header + multiple lines) and a "freestyle" (hand placed
      widgets): not sure how other UIs will work yet (HUD? maybe just more widgets, trying to not
      generalize)
  - [ ] UI stack? This will be needed if we want a Monster Hunter way of doing UI (controlling the
      character works fine when UI is opened, but only the top of the UI stack accepts UI input)
  - [ ] Position/size should be calculated on rendering call. The calls should store the layout and
      state (this makes the rendering and interactions delayed by 1 frame, which is usually fine)
    - [ ] Layout: size (auto+min+max, fixed)
    - [ ] Extra commands: indent, unindent, same line, (group start/end? -> maybe this will require
        two passes: calculate position/size, render)
    - [ ] Animations
  - [ ] Widgets
    - [x] Text
      - [ ] Allow more sizes?
      - [ ] Centered
      - [ ] Clickable (hover style, down style, etc)
    - [ ] Combobox
      - [ ] Combowheel?
      - [ ] Left/right control (seems to be the simplest and better control)
    - [ ] Separator
  - [ ] Keyboard/Controller support
    - [ ] Styling colors for text/widgets (or colored background of the line)
  - [ ] Styling options
    - [ ] Split style into multiple sections (placer options can be easily copiable, for example)
  - [ ] Custom render shader
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
- [ ] [vfs] Virtual file system (like physicsFS)

### Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image
