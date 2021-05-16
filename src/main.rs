struct Color {
    r: u8,
    g: u8,
    b: u8
}

fn complementary(color: &Color) -> Color {
    Color {
        r: 255 - color.r,
        g: 255 - color.g,
        b: 255 - color.b
    }
}

fn complementary_in_place(color: &Color) -> Color {
    let com_in_place = Color {r: 255 - color.r, g: 255 - color.g, b: 255 - color.b};
    //color.r = 255 - color.r;
    //color.g = 255 - color.g;
    //color.b = 255 - color.b;
    return com_in_place;
}

fn display(color: Color) {
    println!("{:x} {:x} {:x}", color.r, color.g, color.b);
}

fn main() {
    let red = Color { r: 255, g: 0, b: 0 };
    display(complementary(&red));

    complementary_in_place(&red);
    display(red);
}
