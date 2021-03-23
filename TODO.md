# General

- [x] Create imgui macro to draw structs
- [ ] rust-sdl2 subsystems should be copied instead of referenced. We may refactor a lot of the app
    code

# Engine

- [ ] Rename animations, time and tasks to *_system
- [ ] Asset system
- [ ] Input system
  - [ ] Mapping
  - [x] Virtual button
    - [x] Keyboard
    - [x] Mouse
    - [x] Controller button
    - [x] Controller axis
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
    - [x] Rumble normal
    - [ ] Dualsense extra feedbacks
- [ ] [render]
  - [ ] Font rendering
  - [ ] Batch rendering
  - [ ] Shader struct
  - [ ] Render to framebuffer + post render effects
  - [ ] verify gl errors
- [ ] Test all parts
- [ ] [ui] Command buffer "immediate mode"
- [ ] [debug] imgui architecture make it not possible to pass App down to callbacks
- [ ] [debug] rename to Editor and implement an Immediate Mode GUI from scratch (or use the, to be
    implemented, UI system)

# Game

# Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image

# Issues

- [ ] [input system] controller shouldn't panic on not connected, only check (axis motion events can
        happen at the same time the controller disconnects)
