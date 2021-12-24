mod hard_drop_info;
mod playfield_preview;

use super::*;

pub use playfield_preview::*;
use hard_drop_info::*;

#[derive(Debug, ImDraw)]
pub enum FocusedRule {
    RotationSystem,

    HardDrop(PlayfieldAnimation),
    HardDropLock,

    SoftDrop,
    SoftDropLock,

    LockDelayRule,

    Ghost,
    Hold,
    HoldResetRotation,

    InitialRotation,
    InitialHold,
    SpawnDrop,
    SpawnRow,
    NextPiecesPreview,

    LineClearRule,
    TopOutRule,

    DelayedAutoShift,
    AutoRepeatRate,

    SoftDropInterval,
    LineClearDelay,

    GravityCurve,
    ScoringCurve,
    LevelCurve,
    StartLevel,
    MinimumLevel,

    EntryDelay,

    RandomizerType,
}

impl MainMenuScene {
    pub fn show_custom_rules(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        //let window_size = app.window_size();
        //let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 40, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout)
            .fixed_height()
            .build(app);
        ui::Text::builder("CUSTOM GAME").build(app);

        let mut rules_box_placer = ui::PagedBox::builder(26).build(app).unwrap();
        {
            // @TODO combobox for presets
            let mut preset_index = 0;
            ui::Combobox::builder("PRESET", &["(CUSTOM)"])
                .build_with_placer(&mut preset_index, &mut rules_box_placer, app);

            // Rotation Systems
            // @TODO ComboboxEnum
            let mut rotation_system = match self.custom_rules.rotation_system {
                RotationSystem::Original => 0,
                RotationSystem::NRSL     => 1,
                RotationSystem::NRSR     => 2,
                RotationSystem::Sega     => 3,
                RotationSystem::ARS      => 4,
                RotationSystem::SRS      => 5,
                RotationSystem::DTET     => 6,
            };
            let state = ui::Combobox::builder("ROTATION SYSTEM", ROTATION_SYSTEM_NAMES)
                .build_with_placer(&mut rotation_system, &mut rules_box_placer, app);
            self.custom_rules.rotation_system = match rotation_system {
                0 => RotationSystem::Original,
                1 => RotationSystem::NRSL,
                2 => RotationSystem::NRSR,
                3 => RotationSystem::Sega,
                4 => RotationSystem::ARS,
                5 => RotationSystem::SRS,
                _ => RotationSystem::DTET,
            };

            if state.focused { self.focused_rule = Some(FocusedRule::RotationSystem); }

            // Hard drop

            let state = ui::Checkbox::builder("HARD DROP")
                .build_with_placer(&mut self.custom_rules.has_hard_drop, &mut rules_box_placer, app);

            if state.focused {
                if let Some(FocusedRule::HardDrop(_)) = self.focused_rule {
                    // keep
                } else {
                    self.focused_rule = Some(FocusedRule::HardDrop(HardDropPreview::new()));
                }
            }

            ui::Checkbox::builder("  HARD DROP LOCK")
                .disabled(!self.custom_rules.has_hard_drop)
                .build_with_placer(&mut self.custom_rules.has_hard_drop_lock, &mut rules_box_placer, app);

            // Soft drop

            let state = ui::Checkbox::builder("SOFT DROP")
                .build_with_placer(&mut self.custom_rules.has_soft_drop, &mut rules_box_placer, app);

            if state.focused { self.focused_rule = Some(FocusedRule::SoftDrop); }

            ui::Checkbox::builder("  SOFT DROP LOCK")
                .disabled(!self.custom_rules.has_soft_drop)
                .build_with_placer(&mut self.custom_rules.has_soft_drop_lock, &mut rules_box_placer, app);

            ui::SliderU64::builder("  INTERVAL", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.soft_drop_interval, &mut rules_box_placer, app);

            ui::Checkbox::builder("HOLD PIECE")
                .build_with_placer(&mut self.custom_rules.has_hold_piece, &mut rules_box_placer, app);
            ui::Checkbox::builder("  RESET ROTATION")
                .disabled(!self.custom_rules.has_hold_piece)
                .build_with_placer(&mut self.custom_rules.hold_piece_reset_rotation, &mut rules_box_placer, app);

            ui::Checkbox::builder("GHOST PIECE")
                .build_with_placer(&mut self.custom_rules.has_ghost_piece, &mut rules_box_placer, app);

            ui::Checkbox::builder("SPAWN DROP")
                .build_with_placer(&mut self.custom_rules.spawn_drop, &mut rules_box_placer, app);

            ui::Checkbox::builder("IRS")
                .build_with_placer(&mut self.custom_rules.has_initial_rotation_system, &mut rules_box_placer, app);
            ui::Checkbox::builder("IHS")
                .build_with_placer(&mut self.custom_rules.has_initial_hold_system, &mut rules_box_placer, app);

            ui::SliderU8::builder("SPAWN ROW", 0, 24)
                .build_with_placer(&mut self.custom_rules.spawn_row, &mut rules_box_placer, app);
            ui::SliderU8::builder("NEXT PIECES", 0, 8)
                .build_with_placer(&mut self.custom_rules.next_pieces_preview_count, &mut rules_box_placer, app);

            // @TODO ComboboxEnum
            let mut line_clear = match self.custom_rules.line_clear_rule {
                LineClearRule::Naive   => 0,
                LineClearRule::Sticky  => 1,
                LineClearRule::Cascade => 2,
            };
            ui::Combobox::builder("LINE CLEAR", LINE_CLEAR_RULE_NAMES)
                .build_with_placer(&mut line_clear, &mut rules_box_placer, app);
            self.custom_rules.line_clear_rule = match line_clear {
                0 => LineClearRule::Naive,
                1 => LineClearRule::Sticky,
                _ => LineClearRule::Cascade,
            };

            ui::SliderU64::builder("  DELAY", 0, 1_000_000)
                .build_with_placer(&mut self.custom_rules.line_clear_delay, &mut rules_box_placer, app);

            ui::Text::builder("TOP OUT RULE")
                .build_with_placer(&mut rules_box_placer, app);

            let mut block_out = self.custom_rules.top_out_rule.contains(TopOutRule::BLOCK_OUT);
            let block_out_state = ui::Checkbox::builder("  BLOCK OUT")
                .build_with_placer(&mut block_out, &mut rules_box_placer, app);
            if block_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::BLOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::BLOCK_OUT);
                }
            }

            let mut lock_out = self.custom_rules.top_out_rule.contains(TopOutRule::LOCK_OUT);
            let lock_out_state = ui::Checkbox::builder("  LOCK OUT").
                build_with_placer(&mut lock_out, &mut rules_box_placer, app);
            if lock_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::LOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::LOCK_OUT);
                }
            }

            let mut partial_lock_out = self.custom_rules.top_out_rule.contains(TopOutRule::PARTIAL_LOCK_OUT);
            let partial_lock_out_state = ui::Checkbox::builder("  PARTIAL LOCK OUT")
                .build_with_placer(&mut partial_lock_out, &mut rules_box_placer, app);
            if partial_lock_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::PARTIAL_LOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::PARTIAL_LOCK_OUT);
                }
            }

            let mut garbage_out = self.custom_rules.top_out_rule.contains(TopOutRule::GARBAGE_OUT);
            let garbage_out_state = ui::Checkbox::builder("  GARBAGE OUT")
                .build_with_placer(&mut garbage_out, &mut rules_box_placer, app);
            if garbage_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::GARBAGE_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::GARBAGE_OUT);
                }
            }

            // @TODO ui for time values (in frames?)
            // @TODO slider float
            ui::SliderU64::builder("DAS", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.das_repeat_delay, &mut rules_box_placer, app);
            ui::SliderU64::builder("ARR", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.das_repeat_interval, &mut rules_box_placer, app);

            //pub gravity_curve: GravityCurve,
            //pub scoring_curve: ScoringRule,
            //pub level_curve: LevelCurve, // @Maybe rename to difficulty curve

            ui::SliderU64::builder("ENTRY DELAY", 0, 2_000_000)
                .build_with_placer(&mut self.custom_rules.entry_delay, &mut rules_box_placer, app);

            ui::SliderU8::builder("START LEVEL", self.custom_rules.minimum_level, 50)
                .build_with_placer(&mut self.custom_rules.minimum_level, &mut rules_box_placer, app);

            //pub lock_delay: LockDelayRule,

            //pub randomizer_type: RandomizerType,
            //app.input_u64_stretch("seed", &mut self.seed);
        }

        if ui::Button::new("START GAME", app).pressed {
            self.state = State::Custom;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }

        self.show_custom_rules_preview(app, _persistent);
    }

    pub fn show_custom_rules_preview(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        match &mut self.focused_rule {
            Some(focused_rule) => {
                match focused_rule {
                    // @TODO remove self ref from all
                    FocusedRule::RotationSystem => self.show_custom_rules_info_rotation_system(app, _persistent),
                    FocusedRule::HardDrop(preview) => show_custom_rules_info_hard_drop(preview, app, _persistent),
                    //FocusedRule::SoftDrop     => self.show_custom_rules_info_soft_drop(app, _persistent),
                    _                           => self.show_custom_rules_info(app, _persistent),
                }
            }

            None => self.show_custom_rules_info(app, _persistent),
        }

        /*
        RotationSystem,

        HardDrop,
        HardDropLock,

        SoftDrop,
        SoftDropLock,

        LockDelayRule,

        Ghost,
        Hold,
        HoldResetRotation,

        InitialRotation,
        InitialHold,
        SpawnDrop,
        SpawnRow,
        NextPiecesPreview,

        LineClearRule,
        TopOutRule,

        DelayedAutoShift,
        AutoRepeatRate,

        SoftDropInterval,
        LineClearDelay,

        GravityCurve,
        ScoringCurve,
        LevelCurve,
        StartLevel,
        MinimumLevel,

        EntryDelay,

        RandomizerType,
        */
    }

    pub fn show_custom_rules_info(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 660, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout).fixed_height().build(app);
        ui::Text::builder("RULE DESCRIPTION").build(app);
    }

    fn show_custom_rules_info_rotation_system(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 660, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout).fixed_height().build(app);
        ui::Text::builder("ROTATION SYSTEM").build(app);
        ui::Text::builder("NINTENDO ROTATION SYSTEM RIGHTHAND").build(app);
    }
}
