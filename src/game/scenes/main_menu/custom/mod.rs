mod ghost_info;
mod hard_drop_info;
mod rotation_system_info;
mod soft_drop_info;
mod soft_drop_interval_info;
mod spawn_delay_info;
mod playfield_animation;

use super::*;

use ghost_info::*;
use hard_drop_info::*;
use rotation_system_info::*;
use soft_drop_info::*;
use soft_drop_interval_info::*;
use spawn_delay_info::*;
pub use playfield_animation::*;

#[derive(Debug, ImDraw)]
pub enum FocusedRule {
    RotationSystem,

    HardDrop(PlayfieldAnimation),
    HardDropLock,

    SoftDrop(PlayfieldAnimation),
    SoftDropLock,
    SoftDropInterval(PlayfieldAnimation),

    LockDelayRule,

    Hold,
    HoldResetRotation,

    Ghost(PlayfieldAnimation),

    SpawnRow,
    SpawnDrop,
    SpawnDelay,
    InitialRotation,
    InitialHold,

    NextPiecesPreview,

    LineClearRule,
    LineClearDelay,

    TopOutBlockOut,
    TopOutLockOut,
    TopOutPartialLockOut,
    TopOutGarbageOut,

    DelayedAutoShift,
    AutoRepeatRate,

    GravityCurve,
    ScoringCurve,
    LevelCurve,
    StartLevel,
    MinimumLevel,

    RandomizerType,
}

macro_rules! change_rule_info {
    ($self:ident, $state:ident, $rule:ident) => {
        if $state.focused {
            $self.focused_rule = Some(FocusedRule::$rule);
        }
    };

    ($self:ident, $state:ident, $rule:ident, $preview:expr) => {
        if $state.focused {
            if let Some(FocusedRule::$rule(_)) = $self.focused_rule {
                // keep
            } else {
                $self.focused_rule = Some(FocusedRule::$rule($preview));
            }
        }
    };
}

macro_rules! change_rule_info_on_change {
    ($self:ident, $state:ident, $rule:ident, $preview:expr) => {
        if $state.focused {
            if $state.changed {
                $self.focused_rule = Some(FocusedRule::$rule($preview));
            } else {
                if let Some(FocusedRule::$rule(_)) = $self.focused_rule {
                    // keep
                } else {
                    $self.focused_rule = Some(FocusedRule::$rule($preview));
                }
            }
        }
    };
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

        let mut rules_box_placer = ui::PagedBox::builder(25).build(app).unwrap();
        {
            // @TODO combobox for presets
            let mut preset_index = 0;
            ui::Combobox::builder("PRESET", &["(CUSTOM)"])
                .build_with_placer(&mut preset_index, &mut rules_box_placer, app);

            //change_rule_info!(self, state, Preset);

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

            change_rule_info!(self, state, RotationSystem);

            // Hard drop

            let state = ui::Checkbox::builder("HARD DROP")
                .build_with_placer(&mut self.custom_rules.has_hard_drop, &mut rules_box_placer, app);

            change_rule_info!(self, state, HardDrop, HardDropPreview::new());

            let state = ui::Checkbox::builder("  FIRM DROP")
                //.disabled(!self.custom_rules.has_hard_drop)
                .disabled(true)
                .build_with_placer(&mut self.custom_rules.has_hard_drop_lock, &mut rules_box_placer, app);

            change_rule_info!(self, state, HardDropLock);

            // Soft drop

            let state = ui::Checkbox::builder("SOFT DROP")
                .build_with_placer(&mut self.custom_rules.has_soft_drop, &mut rules_box_placer, app);

            change_rule_info!(self, state, SoftDrop, SoftDropPreview::new());

            let state = ui::Checkbox::builder("  SOFT DROP LOCK")
                //.disabled(!self.custom_rules.has_soft_drop)
                .disabled(true)
                .build_with_placer(&mut self.custom_rules.has_soft_drop_lock, &mut rules_box_placer, app);

            change_rule_info!(self, state, SoftDropLock);

            let state = ui::SliderU64::builder("  INTERVAL", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.soft_drop_interval, &mut rules_box_placer, app);

            change_rule_info_on_change!(
                self, state, SoftDropInterval, SoftDropIntervalPreview::new(self.custom_rules.clone())
            );

            // Hold piece

            let state = ui::Checkbox::builder("HOLD PIECE")
                .build_with_placer(&mut self.custom_rules.has_hold_piece, &mut rules_box_placer, app);

            change_rule_info!(self, state, Hold);

            let state = ui::Checkbox::builder("  RESET ROTATION")
                .disabled(!self.custom_rules.has_hold_piece)
                .build_with_placer(&mut self.custom_rules.hold_piece_reset_rotation, &mut rules_box_placer, app);

            change_rule_info!(self, state, HoldResetRotation);

            // Ghost

            let state = ui::Checkbox::builder("GHOST PIECE")
                .build_with_placer(&mut self.custom_rules.has_ghost_piece, &mut rules_box_placer, app);

            change_rule_info_on_change!(
                self, state, Ghost, GhostPreview::new(self.custom_rules.has_ghost_piece)
            );

            // Spawn

            let state = ui::SliderU8::builder("SPAWN ROW", 0, 24)
                .build_with_placer(&mut self.custom_rules.spawn_row, &mut rules_box_placer, app);

            change_rule_info!(self, state, SpawnRow);

            let state = ui::Checkbox::builder("SPAWN DROP")
                .build_with_placer(&mut self.custom_rules.spawn_drop, &mut rules_box_placer, app);

            change_rule_info!(self, state, SpawnDrop);

            let state = ui::SliderU64::builder("SPAWN DELAY / ARE", 0, 2_000_000)
                .build_with_placer(&mut self.custom_rules.spawn_delay, &mut rules_box_placer, app);

            change_rule_info!(self, state, SpawnDelay);

            // @TODO set IRS and IHS to false automatically when ARE is 0
            let state = ui::Checkbox::builder("IRS")
                .disabled(self.custom_rules.spawn_delay == 0)
                .build_with_placer(&mut self.custom_rules.has_initial_rotation_system, &mut rules_box_placer, app);

            change_rule_info!(self, state, InitialRotation);

            let state = ui::Checkbox::builder("IHS")
                .disabled(self.custom_rules.spawn_delay == 0)
                .build_with_placer(&mut self.custom_rules.has_initial_hold_system, &mut rules_box_placer, app);

            change_rule_info!(self, state, InitialHold);

            // Next pieces

            let state = ui::SliderU8::builder("NEXT PIECES", 0, 8)
                .build_with_placer(&mut self.custom_rules.next_pieces_preview_count, &mut rules_box_placer, app);

            change_rule_info!(self, state, NextPiecesPreview);

            // Line clear

            // @TODO ComboboxEnum
            let mut line_clear = match self.custom_rules.line_clear_rule {
                LineClearRule::Naive   => 0,
                LineClearRule::Sticky  => 1,
                LineClearRule::Cascade => 2,
            };
            let state = ui::Combobox::builder("LINE CLEAR", LINE_CLEAR_RULE_NAMES)
                .build_with_placer(&mut line_clear, &mut rules_box_placer, app);
            self.custom_rules.line_clear_rule = match line_clear {
                0 => LineClearRule::Naive,
                1 => LineClearRule::Sticky,
                _ => LineClearRule::Cascade,
            };

            change_rule_info!(self, state, LineClearRule);

            let state = ui::SliderU64::builder("  DELAY", 0, 1_000_000)
                .build_with_placer(&mut self.custom_rules.line_clear_delay, &mut rules_box_placer, app);

            change_rule_info!(self, state, LineClearDelay);

            // Top out

            ui::Text::builder("TOP OUT RULE")
                .build_with_placer(&mut rules_box_placer, app);

            let mut block_out = self.custom_rules.top_out_rule.contains(TopOutRule::BLOCK_OUT);
            let state = ui::Checkbox::builder("  BLOCK OUT")
                .build_with_placer(&mut block_out, &mut rules_box_placer, app);
            if state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::BLOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::BLOCK_OUT);
                }
            }

            change_rule_info!(self, state, TopOutBlockOut);

            let mut lock_out = self.custom_rules.top_out_rule.contains(TopOutRule::LOCK_OUT);
            let state = ui::Checkbox::builder("  LOCK OUT").
                build_with_placer(&mut lock_out, &mut rules_box_placer, app);
            if state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::LOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::LOCK_OUT);
                }
            }

            change_rule_info!(self, state, TopOutLockOut);

            let mut partial_lock_out = self.custom_rules.top_out_rule.contains(TopOutRule::PARTIAL_LOCK_OUT);
            let state = ui::Checkbox::builder("  PARTIAL LOCK OUT")
                .build_with_placer(&mut partial_lock_out, &mut rules_box_placer, app);
            if state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::PARTIAL_LOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::PARTIAL_LOCK_OUT);
                }
            }

            change_rule_info!(self, state, TopOutPartialLockOut);

            let mut garbage_out = self.custom_rules.top_out_rule.contains(TopOutRule::GARBAGE_OUT);
            let state = ui::Checkbox::builder("  GARBAGE OUT")
                .build_with_placer(&mut garbage_out, &mut rules_box_placer, app);
            if state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::GARBAGE_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::GARBAGE_OUT);
                }
            }

            change_rule_info!(self, state, TopOutGarbageOut);

            // DAS + ARR

            // @TODO ui for time values (in frames?)
            // @TODO slider float
            let state = ui::SliderU64::builder("DAS", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.das_repeat_delay, &mut rules_box_placer, app);

            change_rule_info!(self, state, DelayedAutoShift);

            let state = ui::SliderU64::builder("ARR", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.das_repeat_interval, &mut rules_box_placer, app);

            change_rule_info!(self, state, AutoRepeatRate);

            // Curves

            //pub gravity_curve: GravityCurve,
            //pub scoring_curve: ScoringRule,
            //pub level_curve: LevelCurve, // @Maybe rename to difficulty curve

            // Start level

            let state = ui::SliderU8::builder("START LEVEL", self.custom_rules.minimum_level, 50)
                .build_with_placer(&mut self.custom_rules.minimum_level, &mut rules_box_placer, app);

            change_rule_info!(self, state, AutoRepeatRate);

            //pub lock_delay: LockDelayRule,

            //pub randomizer_type: RandomizerType,
            //app.input_u64_stretch("seed", &mut self.seed);
        }

        if ui::Button::new("PREVIEW", app).pressed {
            self.state = State::CustomPreview;
        }

        if ui::Button::new("START GAME", app).pressed {
            self.state = State::CustomLocal;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }

        self.show_custom_rules_preview(app, _persistent);
    }

    pub fn show_custom_rules_preview(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        match &mut self.focused_rule {
            Some(focused_rule) => {
                match focused_rule {
                    // @TODO remove self ref from all
                    FocusedRule::RotationSystem =>
                        show_custom_rules_info_rotation_system(app, persistent),

                    FocusedRule::HardDrop(preview) =>
                        show_custom_rules_info_hard_drop(preview, app, persistent),
                    FocusedRule::SoftDrop(preview) =>
                        show_custom_rules_info_soft_drop(preview, app, persistent),

                    FocusedRule::SoftDropInterval(preview) =>
                        show_custom_rules_info_soft_drop_interval(preview, app, persistent),

                    FocusedRule::Ghost(preview) =>
                        show_custom_rules_info_ghost(preview, app, persistent),

                    FocusedRule::SpawnDelay =>
                        show_custom_rules_info_spawn_delay(app, persistent),

                    _ => self.show_custom_rules_info(app, persistent),
                }
            }

            None => self.show_custom_rules_info(app, persistent),
        }
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
}
