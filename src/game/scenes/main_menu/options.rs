use super::*;

impl MainMenuScene {
    pub fn show_options(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 300 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS").build(app);

        if ui::Button::new("VIDEO", app).pressed {
            self.state = State::OptionsVideo;
        }

        if ui::Button::new("AUDIO", app).pressed {
            self.state = State::OptionsAudio;
        }

        if ui::Button::new("CONTROLS", app).pressed {
            self.state = State::OptionsControls;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }
    }

    pub fn show_options_video(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        // @TODO cache strings and dyn arrays

        // @Important reloading video info every time. In case we don't we need to check which UI
        // elements changed to know what/when to reload this info
        self.video_info = VideoInfo::load(app);

        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 300 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };

        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS - VIDEO").build(app);

        //Text::new("OPTIONS - VIDEO", app);

        // @TODO most video options should have a confirm button in 15 sec or change back if the
        //       the user can't click it. This will avoid issues

        // Fullscreen options
        let mut screen_mode_index = match self.video_info.screen_mode {
            FullscreenType::Off     => 0,
            FullscreenType::True    => 1,
            FullscreenType::Desktop => 2,
        };

        let screen_mode_combobox =
            ui::Combobox::builder("SCREEN MODE", &SCREEN_MODES).build(&mut screen_mode_index, app);

        if screen_mode_combobox.changed {
            self.video_info.screen_mode = match screen_mode_index {
                0 => FullscreenType::Off,
                1 => FullscreenType::True,
                _ => FullscreenType::Desktop,
            };

            app.set_window_screen_mode(self.video_info.screen_mode);
        }

        // Display
        let num_displays = app.num_displays();
        let mut display_index = self.video_info.display_index as usize;
        let display_state = ui::Combobox::builder("DISPLAY", &DISPLAY_NAMES[..num_displays])
            .build(&mut display_index, app);

        if display_state.changed {
            app.move_window_to_display(display_index as u32);
        }

        // Resolution and refresh rate
        let window_sizes_and_rates = app.available_window_sizes_and_rates();
        let is_fullscreen = app.window_screen_mode() == FullscreenType::True;
        let is_desktop    = app.window_screen_mode() == FullscreenType::Desktop;

        // Resolution
        let current_size = (self.video_info.window_size.0, self.video_info.window_size.1);

        let mut window_sizes_strs = window_sizes_and_rates
            .iter()
            .map(|size_and_rate| format!("{}x{}", size_and_rate.0.0, size_and_rate.0.1))
            .collect::<Vec<String>>();

        let mut size_index = window_sizes_and_rates
            .iter()
            .position(|size_and_rate| size_and_rate.0 == current_size)
            .unwrap_or_else(|| {
                window_sizes_strs.push("(CUSTOM)".to_owned());
                window_sizes_strs.len()-1
            });

        let window_size_state = ui::Combobox::builder("RESOLUTION", &window_sizes_strs)
            .disabled(is_desktop)
            .build(&mut size_index, app);

        if window_size_state.changed {
            // size_index == len only if it was manually resized (or OS resized it)
            if size_index < window_sizes_and_rates.len() {
                let size = window_sizes_and_rates[size_index].0;

                if !is_fullscreen {
                    app.set_window_size(size.0, size.1);
                } else {
                    if !(window_sizes_and_rates[size_index].1).contains(&(self.video_info.display_mode.refresh_rate as u32)) {
                        self.video_info.display_mode.refresh_rate = (window_sizes_and_rates[size_index].1)[0] as i32;
                    }

                    self.video_info.display_mode.w = size.0 as i32;
                    self.video_info.display_mode.h = size.1 as i32;
                    app.set_window_display_mode(self.video_info.display_mode);
                }
            }
        }

        // Refresh rate
        let refresh_rates_strs;
        let mut rate_index;
        if is_fullscreen && size_index < window_sizes_and_rates.len() {
            rate_index = window_sizes_and_rates[size_index].1
                .iter()
                .position(|&rate| rate == self.video_info.display_mode.refresh_rate as u32)
                .unwrap();

            refresh_rates_strs = window_sizes_and_rates[size_index].1
                .iter()
                .map(|rate| format!("{} Hz", rate))
                .collect::<Vec<String>>();
        } else {
            rate_index = 0;
            refresh_rates_strs = vec![format!("{} Hz", self.video_info.display_mode.refresh_rate)];
        }

        let refresh_rates_state = ui::Combobox::builder("REFRESH RATE", &refresh_rates_strs)
            .disabled(!is_fullscreen)
            .build(&mut rate_index, app);

        if refresh_rates_state.changed && is_fullscreen {
            let rate = (window_sizes_and_rates[size_index].1)[rate_index];
            self.video_info.display_mode.refresh_rate = rate as i32;
            app.set_window_display_mode(self.video_info.display_mode);
        }

        // VSync
        let mut vsync            = self.video_info.vsync != SwapInterval::Immediate;
        let mut adaptative_vsync = self.video_info.vsync == SwapInterval::LateSwapTearing;

        let vsync_checkbox =
            ui::Checkbox::builder("VSYNC")
            .build(&mut vsync, app);

        let adaptative_vsync_checkbox =
            ui::Checkbox::builder("  ADAPTATIVE")
            .disabled(!vsync)
            .build(&mut adaptative_vsync, app);

        if vsync_checkbox.changed || adaptative_vsync_checkbox.changed {
            self.video_info.vsync = if vsync {
                if adaptative_vsync {
                    SwapInterval::LateSwapTearing
                } else {
                    SwapInterval::VSync
                }
            } else {
                SwapInterval::Immediate
            };

            app.set_vsync(self.video_info.vsync);
        }

        /*
        if ui::Button::new("APPLY", app).pressed {
        }
        */

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Options;
        }
    }

    pub fn show_options_audio(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 300 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS - AUDIO").build(app);

        // Music volume
        let mut music_volume = (app.music_volume() * 255.0) as i32;
        let music_volume_slider =
            ui::SliderI32::builder("MUSIC", 0, 255)
            .build(&mut music_volume, app);

        if music_volume_slider.changed {
            app.set_music_volume(music_volume as f32 / 255.0);
        }

        // SFX volume
        let mut sfx_volume = (app.sfx_volume() * 255.0) as i32;
        let sfx_volume_slider =
            ui::SliderI32::builder("SFX", 0, 255)
                .build(&mut sfx_volume, app);

        if sfx_volume_slider.changed {
            app.set_sfx_volume(sfx_volume as f32 / 255.0);
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Options;
        }
    }

    pub fn show_options_controls(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 300 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS - CONTROLS").build(app);

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Options;
        }
    }
}
