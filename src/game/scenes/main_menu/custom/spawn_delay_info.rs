use super::*;
use crate::game::{
    input::*,
    pieces::PieceVariant,
};

/*
pub struct SpawnDelayPreview;

impl SpawnDelayPreview {
    // @TODO copy current rules from the menu
    pub fn new() -> PlayfieldAnimation {
        // @Maybe using only SRS for previews for now, maybe we should show the rotation system the
        //        player has chosen?
        let mut rules: Rules = RotationSystem::SRS.into();

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

        let sequence_builder = InputSequenceBuilder::new()
            // L
            .wait(300_000)
            .click(KEY_RIGHT)
            .wait(300_000)
            .click(KEY_HARD_DROP)

            // S
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_HARD_DROP)

            .wait(rules.line_clear_delay)

            // Z
            .wait(300_000)
            .click(KEY_ROTATE_CCW)
            .wait(300_000)
            .click(KEY_RIGHT)
            .wait(300_000)
            .click(KEY_RIGHT)
            .wait(300_000)
            .click(KEY_HARD_DROP)

            // L
            .wait(300_000)
            .click(KEY_ROTATE_CW)
            .wait(300_000)
            .click(KEY_ROTATE_CW)
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_HARD_DROP)

            .wait(rules.line_clear_delay)

            // I
            .wait(300_000)
            .click(KEY_LEFT)
            .wait(300_000)
            .click(KEY_HARD_DROP)

            .wait(rules.line_clear_delay)


            .build();

        PlayfieldAnimation::new(rules, playfield, randomizer, sequence_builder)
    }
}
*/

pub fn show_custom_rules_info_spawn_delay(
    //preview: &mut PlayfieldAnimation,
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
    ui::Text::builder("SPAWN DELAY").build(app);

    let text =
        "ARE, also called entry delay, appearance delay, or spawn delay, is the period of time \
        between the lockdown of one tetromino and the appearance of the next tetromino.";
    ui::Text::builder(text).multiline(true).build(app);

    let text =
        "Generally, during ARE, the player must wait for the next tetromino to appear, and cannot \
        actively cause anything to happen. However, ARE can serve as a time for the player to \
        input DAS, IRS and IHS actions which will be processed in the moment ARE finishes and the \
        next tetromino appears.";
    ui::Text::builder(text).multiline(true).build(app);

    /*
    // Render example playfield
    let mut batch = Batch::new();

    // Playfield animation
    let last_frame_duration = app.last_frame_duration();
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
        "main_menu/custom/hard_drop/playfield",
        playfield_draw_size.x as u32,
        playfield_draw_size.y as u32,
        None
    );

    let framebuffer = app.get_framebuffer_or_create(
        "main_menu/custom/hard_drop/playfield",
        texture
    );
    framebuffer.clear(BufferClear::new().color(color::TRANSPARENT));

    app.render_batch(batch, Some(framebuffer));
    ui::Texture::new(texture, app);
    */
}
