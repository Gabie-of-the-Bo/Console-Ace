use std::io::{stdout, Write};

use crossterm::{cursor::{Hide, MoveTo, Show}, event::{DisableMouseCapture, EnableMouseCapture}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen, SetSize}, ExecutableCommand};

pub fn hide_cursor() {
    let mut out = stdout();
    
    out.execute(Hide).expect("Unable to hide cursor");
    flush();
}

pub fn show_cursor() {
    let mut out = stdout();

    out.execute(Show).expect("Unable to show cursor");
    flush();
}

pub fn enable_mouse_capture() {
    let mut out = stdout();

    out.execute(EnableMouseCapture).expect("Unable to enable mouse capture");
    flush();
}

pub fn disable_mouse_capture() {
    let mut out = stdout();

    out.execute(DisableMouseCapture).expect("Unable to disable mouse capture");
    flush();
}

pub fn enter_alternate_screen() {
    let mut out = stdout();

    out.execute(EnterAlternateScreen).expect("Unable to enter alternate screen");
}

pub fn leave_alternate_screen() {
    let mut out = stdout();

    out.execute(LeaveAlternateScreen).expect("Unable to leave alternate screen");
}

pub fn clear() {
    let mut out = stdout();

    out.execute(Clear(crossterm::terminal::ClearType::All)).expect("Unable to clear console");
}

pub fn clear_section(r0: usize, c0: usize, r1: usize, c1: usize) {
    // Compute width and height (inclusive)
    let width = c1.saturating_sub(c0) + 1;
    let height = r1.saturating_sub(r0) + 1;

    for row in r0..r0 + height {
        // Move to start of this row
        move_cursor(row, c0);
        write_str(&" ".repeat(width));
    }
}

pub fn resize(rows: usize, cols: usize) {
    let mut out = stdout();

    out.execute(SetSize(cols as u16, rows as u16)).expect("Unable to resize console");
}

pub fn move_cursor(row: usize, col: usize) {
    let mut out = stdout();

    out.execute(MoveTo(col as u16, row as u16)).expect("Unable to move cursor");
}

pub fn write_str(string: &str) {
    let mut out = stdout();
    
    out.execute(Print(string)).expect("Unable to write to console");
}

pub fn write_char(char: char) {
    let mut out = stdout();
    
    out.execute(Print(char)).expect("Unable to write to console");
}

pub fn write_char_color(char: char, background: Color, font: Color) {
    let mut out = stdout();
    
    execute!(
        out,
        SetBackgroundColor(background),
        SetForegroundColor(font),
        Print(char),
        ResetColor
    ).expect("Unable to write to console");
}

pub fn set_color(background: Color, font: Color) {
    let mut out = stdout();
    
    execute!(
        out,
        SetBackgroundColor(background),
        SetForegroundColor(font),
    ).expect("Unable to update color");
}

pub fn draw_bar(row: usize, col: usize, len: usize, curr: usize, total: usize, color: Color) {
    move_cursor(row, col);

    let percentage = curr as f64 / total as f64;
    let sections = (percentage * len as f64).round() as usize;
    let left = len - sections;
    
    let value_str = format!(" {}/{}", curr, total);
    let mut value_chars = value_str.chars();

    let mut out = stdout();

    execute!(out, SetBackgroundColor(color), SetForegroundColor(Color::Black)).expect("Unable to set color");

    for _ in 0..sections {
        if let Some(char) = value_chars.next() {
            write_char(char);

        } else {
            write_char(' ');
        }
    }

    execute!(out, SetBackgroundColor(Color::Black), SetForegroundColor(Color::White)).expect("Unable to set color");

    for _ in 0..left {
        if let Some(char) = value_chars.next() {
            write_char(char);

        } else {
            write_char(' ');
        }
    }
}

pub fn flush() {
    let mut out = stdout();

    out.flush().expect("Unable to flush stdout");
}

pub fn draw_h_line(row: usize, col: usize, len: usize) {
    move_cursor(row, col);
    write_str(&"─".repeat(len));
}

pub fn draw_h_line_double(row: usize, col: usize, len: usize) {
    move_cursor(row, col);
    write_str(&"═".repeat(len));
}

pub fn draw_h_sep(row: usize, col: usize, len: usize) {
    const LJ: &str = "├";
    const RJ: &str = "┤";

    move_cursor(row, col);
    write_str(LJ);
    draw_h_line(row, col + 1, len.saturating_sub(1));
    write_str(RJ);
}

pub fn draw_v_sep(row: usize, col: usize, len: usize) {
    const UJ: &str = "┬";
    const DJ: &str = "┴";

    move_cursor(row, col);
    write_str(UJ);
    draw_v_line(row + 1, col, len.saturating_sub(1));
    move_cursor(row + len, col);
    write_str(DJ);
}

pub fn draw_v_line(row: usize, col: usize, len: usize) {
    for i in 0..len {
        move_cursor(row + i, col);
        write_str("│");
    }
}

pub fn draw_v_line_double(row: usize, col: usize, len: usize) {
    for i in 0..len {
        move_cursor(row + i, col);
        write_str("║");
    }
}

pub fn draw_square(r0: usize, c0: usize, r1: usize, c1: usize) {
    // Corner characters
    const TL: &str = "┌";
    const TR: &str = "┐";
    const BL: &str = "└";
    const BR: &str = "┘";

    // Dimensions
    let width = c1.checked_sub(c0).unwrap_or(0);
    let height = r1.checked_sub(r0).unwrap_or(0);

    // Top edge
    move_cursor(r0, c0);
    write_str(TL);
    draw_h_line(r0, c0 + 1, width.saturating_sub(1));
    write_str(TR);

    // Sides
    draw_v_line(r0 + 1, c0, height.saturating_sub(1));
    draw_v_line(r0 + 1, c1, height.saturating_sub(1));

    // Bottom edge
    move_cursor(r1, c0);
    write_str(BL);
    draw_h_line(r1, c0 + 1, width.saturating_sub(1));
    write_str(BR);

    // Finally, flush everything in one go
    flush();
}

pub fn draw_square_double(r0: usize, c0: usize, r1: usize, c1: usize) {
    // Corner characters
    const TL: &str = "╔";
    const TR: &str = "╗";
    const BL: &str = "╚";
    const BR: &str = "╝";

    // Dimensions
    let width = c1.checked_sub(c0).unwrap_or(0);
    let height = r1.checked_sub(r0).unwrap_or(0);

    // Top edge
    move_cursor(r0, c0);
    write_str(TL);
    draw_h_line_double(r0, c0 + 1, width.saturating_sub(1));
    write_str(TR);

    // Sides
    draw_v_line_double(r0 + 1, c0, height.saturating_sub(1));
    draw_v_line_double(r0 + 1, c1, height.saturating_sub(1));

    // Bottom edge
    move_cursor(r1, c0);
    write_str(BL);
    draw_h_line_double(r1, c0 + 1, width.saturating_sub(1));
    write_str(BR);

    // Finally, flush everything in one go
    flush();
}

pub fn draw_titled_square(title: &str, r0: usize, c0: usize, r1: usize, c1: usize) {
    let width = c1.checked_sub(c0).unwrap_or(0);

    draw_square(r0, c0, r1, c1);

    move_cursor(r0 + 1, c0 + 1 + width / 2 - title.len() / 2);
    write_str(title);

    draw_h_sep(r0 + 2, c0, width);
}