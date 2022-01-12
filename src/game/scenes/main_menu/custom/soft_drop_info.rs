use super::*;
use crate::game::{
    input::*,
    pieces::PieceVariant,
    rules::GravityCurve,
};

pub struct SoftDropPreview;

impl SoftDropPreview {
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

        let lock_delay = 500_000;

        // @TODO better example

        PlayfieldAnimation::builder()
            // L
            .wait(300_000)
            .click(KEY_RIGHT)
            .wait(300_000)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .wait(lock_delay)

            // S
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .wait(lock_delay)

            .wait(rules.line_clear_delay)

            // Z
            .wait(300_000)
            .click(KEY_ROTATE_CCW)
            .wait(300_000)
            .click(KEY_RIGHT)
            .wait(300_000)
            .click(KEY_RIGHT)
            .wait(300_000)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .wait(lock_delay)

            // L
            .wait(300_000)
            .click(KEY_ROTATE_CW)
            .wait(300_000)
            .click(KEY_ROTATE_CW)
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .wait(lock_delay)

            .wait(rules.line_clear_delay)

            // I
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .click(KEY_SOFT_DROP)
            .wait(lock_delay)

            .wait(rules.line_clear_delay)

            .build(rules, playfield, randomizer)
    }
}

pub fn show_custom_rules_info_soft_drop(
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
    ui::Text::builder("SOFT DROP").build(app);

    let text =
        "A soft drop is a move in which a Tetromino speeds up its downwards motion. It is a higher \
        scoring move than letting the Tetromino fall by itself, but lower scoring than a hard drop.";

    ui::Text::builder(text).multiline(true).build(app);

    // Render example playfield
    let mut batch = Batch::new();

    // Playfield animation
    let last_frame_duration = app.last_frame_real_duration();
    preview.update(last_frame_duration, app);

    preview.instance.update_animations();
    preview.instance.render_playfield(
        Vec2i::new(),
        true,
        &mut batch,
        persistent,
    );

    let playfield = &preview.instance.playfield();
    let playfield_draw_size = get_draw_playfield_size(
        &playfield,
        persistent.pixel_scale,
        true
    );

    let texture = app.get_texture_or_create(
        "main_menu/custom/soft_drop/playfield",
        playfield_draw_size.x as u32,
        playfield_draw_size.y as u32,
        None
    );

    let framebuffer = app.get_framebuffer_or_create(
        "main_menu/custom/soft_drop/playfield",
        texture
    );
    framebuffer.clear(BufferClear::new().color(color::TRANSPARENT));

    app.render_batch(batch, Some(framebuffer));
    ui::Texture::new(texture, app);
}
