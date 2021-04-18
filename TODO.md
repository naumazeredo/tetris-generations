# Tasks

## v0.1

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
  - [x] Feedback
    - [x] Rumble normal
- [x] [render]
  - [x] Font rendering
  - [x] Improve rendering performance
- [ ] Asset system
- [ ] Logger system
- [ ] Refactor systems to match

### Issues

- [x] Rename animations, time and tasks to *_system
- [x] rust-sdl2 subsystems should be copied instead of referenced. We may refactor a lot of the app
    code


## backlog

- [ ] Input system
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

### Game

- [ ] Start game!

### Build system

- [ ] Download Windows SDL2 binaries automatically
- [ ] Cleanup binary dependencies
  - [ ] Maybe use stb_image instead of SDL_image
