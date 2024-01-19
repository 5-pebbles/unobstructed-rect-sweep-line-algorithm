mod rect;
use rect::Rect;

fn main() {
    let rect = Rect::new(0, 0, 20, 7);
    let red = Rect::new(0, 3, 6, 4);
    let blue = Rect::new(10, 0, 10, 2);
    dbg!(rect.unobstructed_rects(&[&red, &blue]));
}
