use super::*;

pub fn show_custom_rules_info_rotation_system(
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

    let text =
        "A rotation system broadly represents where and how tetrominoes spawn, how they rotate, \
        and what wall kicks they may perform.";
    ui::Text::builder(text).multiline(true).build(app);
}
