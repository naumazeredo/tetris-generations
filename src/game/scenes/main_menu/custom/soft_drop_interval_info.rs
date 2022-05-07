use super::*;
use crate::game::{
    input::*,
    pieces::PieceVariant,
    rules::GravityCurve,
};

pub struct SoftDropIntervalPreview;

impl SoftDropIntervalPreview {
    // @TODO copy current rules from the menu
    pub fn new(mut rules: Rules) -> PlayfieldAnimation {
        let drop_interval = 800_000;
        let drop_amount = 10;

        rules.spawn_drop = false;
        rules.spawn_row  = 14;
        rules.gravity_curve = GravityCurve::Fixed(drop_interval);

        let playfield = Playfield::new(Vec2i { x: 5, y: 40 }, drop_amount + 2);

        let randomizer = RandomizerDefinedSequence::new(
            vec![
                PieceVariant::S,
                PieceVariant::L,
                PieceVariant::Z,
                PieceVariant::J,
            ]
        );

        let soft_drop_amount = (drop_amount + 1) as u64;

        PlayfieldAnimation::builder()
            // S
            .wait(soft_drop_amount * drop_interval)

            .new_piece()
            .press(KEY_SOFT_DROP)
            .wait(soft_drop_amount * rules.soft_drop_interval)
            .release(KEY_SOFT_DROP)

            .new_piece()
            .wait(soft_drop_amount * drop_interval)

            .new_piece()
            .press(KEY_SOFT_DROP)
            .wait(soft_drop_amount * rules.soft_drop_interval)
            .release(KEY_SOFT_DROP)

            .build(rules, playfield, randomizer)
    }
}

pub fn show_custom_rules_info_soft_drop_interval(
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
    ui::Text::builder("SOFT DROP INTERVAL").build(app);

    let text = "soft drop interval";

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
    let playfield_draw_size = get_draw_playfield_grid_size(
        Vec2i { x: 5, y: 8 },
        persistent.pixel_scale,
        true
    );

    let texture = app.get_texture_or_create(
        "main_menu/custom/soft_drop_interval/playfield",
        playfield_draw_size.x as u32,
        playfield_draw_size.y as u32,
        None
    );

    let framebuffer = app.get_framebuffer_or_create(
        "main_menu/custom/soft_drop_interval/playfield",
        texture
    );
    framebuffer.clear(BufferClear::new().color(color::TRANSPARENT));

    app.render_batch(batch, Some(framebuffer));
    ui::Texture::new(texture, app);
}
