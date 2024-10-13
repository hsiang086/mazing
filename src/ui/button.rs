use raylib::prelude::*;

pub fn draw_button(d: &mut RaylibDrawHandle, rect: &Rectangle, text: &str, hovered: bool) {
    if hovered {
        d.draw_rectangle_rounded(*rect, 0.5, 10, Color::DARKGRAY);
    } else {
        d.draw_rectangle_rounded(*rect, 0.5, 10, Color::GRAY);
    }
    d.draw_rectangle_lines_ex(*rect, 2.0, Color::BLACK);
    let text_width = d.measure_text(text, 20);
    d.draw_text(
        text,
        (rect.x + (rect.width - text_width as f32) / 2.0) as i32,
        (rect.y + (rect.height - 20.0) / 2.0) as i32,
        20,
        Color::BLACK,
    );
}
