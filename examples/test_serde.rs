use graphics_shapes::rect::Rect;

fn main() {
    let rect = Rect::new((0, 0), (100, 20));
    let str = serde_json::to_string(&rect);
    println!("{}", str.unwrap());
}
