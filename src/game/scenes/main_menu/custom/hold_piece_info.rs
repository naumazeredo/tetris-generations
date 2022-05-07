use super::*;
use crate::game::{
    input::*,
    pieces::PieceVariant,
    rules::GravityCurve,
};

pub struct HoldPiecePreview;

impl HoldPiecePreview {
    // @TODO copy current rules from the menu
    pub fn new() -> PlayfieldAnimation {
        // @Maybe using only SRS for previews for now, maybe we should show the rotation system the
        //        player has chosen?
        let mut rules: Rules = RotationSystem::SRS.into();
        rules.gravity_curve = GravityCurve::NoGravity;

        let playfield = Playfield::new(Vec2i { x: 5, y: 40 }, 8);

        let randomizer = RandomizerDefinedSequence::new(
            vec![
                PieceVariant::L,
                PieceVariant::S,
                PieceVariant::Z,
                PieceVariant::L,
                PieceVariant::I,
            ]
        );

        PlayfieldAnimation::builder()
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_HARD_DROP)

            .wait(rules.line_clear_delay)

            .build(rules, playfield, randomizer)
    }
}

pub fn show_custom_rules_info_hold_piece(
    preview: &mut PlayfieldAnimation,
    app: &mut App,
    persistent: &mut PersistentData
) {
    let menu_size = Vec2i { x: 580, y: 880 };

    // Ui
    let window_layout = ui::Layout {
        pos: Vec2i { x: 660, y: 40, },
        size: menu_size
    };
    ui::Ui::builder(window_layout).fixed_height().build(app);
    ui::Text::builder("HOLD PIECE").build(app);

    // @TODO change this description, it's not good
    let text =
        "At any time starting when a tetromino enters the playfield until it locks, the single \
        player can press the Hold button on the controller to move the active tetromino into the \
        hold space and move the tetromino that was in the hold space to the top of the playfield. \
        A tetromino moved into the hold space is unavailable for switching out until the tetromino \
        that was moved out of the hold space locks.";
    ui::Text::builder(text).multiline(true).build(app);

    // Render example playfield
    let mut batch = Batch::new();

    // Playfield animation
    let last_frame_duration = app.last_frame_real_duration();
    preview.update(last_frame_duration, app);

    preview.tetris_game.update_animations();
    preview.tetris_game.render_playfield(
        Vec2i::new(),
        true,
        &mut batch,
        persistent,
    );

    let playfield = &preview.tetris_game.playfield();
    let playfield_draw_size = get_draw_playfield_size(
        &playfield,
        persistent.pixel_scale,
        true
    );

    let texture = app.get_texture_or_create(
        "main_menu/custom/hold_piece/playfield",
        playfield_draw_size.x as u32,
        playfield_draw_size.y as u32,
        None
    );

    let framebuffer = app.get_framebuffer_or_create(
        "main_menu/custom/hold_piece/playfield",
        texture
    );
    framebuffer.clear(BufferClear::new().color(color::TRANSPARENT));

    app.render_batch(batch, Some(framebuffer));
    ui::Texture::new(texture, app);
}
