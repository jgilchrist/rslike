use gui::chars;
use gui::Console;
use util::units::BorderedRectangle;

pub fn draw_box(console: &mut Console, rect: BorderedRectangle) {
    let loc = rect.location();
    let width = rect.width();
    let height = rect.height();

    console.put_plain(loc + (0, 0), chars::NW);
    console.put_plain(loc + (width, 0), chars::NE);
    console.put_plain(loc + (0, height), chars::SW);
    console.put_plain(loc + (width, height), chars::SE);

    for x in 1..width {
        console.put_plain(loc + (x, 0), chars::HLINE);
        console.put_plain(loc + (x, height), chars::HLINE);
    }

    for y in 1..height {
        console.put_plain(loc + (0, y), chars::VLINE);
        console.put_plain(loc + (width, y), chars::VLINE);
    }
}

pub fn draw_box_with_title(console: &mut Console, title: &str, rect: BorderedRectangle) {
    draw_box(console, rect);
    console.print_plain(rect.location() + (1, 0), title);
}
