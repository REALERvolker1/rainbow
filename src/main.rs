fn main() -> () {
    let mut red_inc = 0;
    let red_ref = 0x10000;
    let mut green_inc = 0;
    let green_ref = 0x100;
    let mut blue_inc = 0;
    let blue_ref = 0x1;

    let mut color = 0xFF0000;
    loop {
        color = color + red_inc + green_inc + blue_inc;
        if color == 0xFF0000 {
            red_inc = 0;
            green_inc = green_ref;
            blue_inc = 0;
        } else if color == 0xFFFF00 {
            red_inc = -red_ref;
            green_inc = 0;
            blue_inc = 0;
        } else if color == 0x00FF00 {
            red_inc = 0;
            green_inc = 0;
            blue_inc = blue_ref;
        } else if color == 0x00FFFF {
            red_inc = 0;
            green_inc = -green_ref;
            blue_inc = 0;
        } else if color == 0x0000FF {
            red_inc = red_ref;
            green_inc = 0;
            blue_inc = 0;
        } else if color == 0xFF00FF {
            red_inc = 0;
            green_inc = 0;
            blue_inc = -blue_ref;
        }
        println!("#{:06X}", color);
    }
}
