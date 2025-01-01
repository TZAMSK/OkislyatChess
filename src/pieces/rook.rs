pub struct Rook;

impl Rook {
    pub fn draw() -> &'static str {
        "\
    \n\
    █▟█▙█\n\
    ▜███▛\n\
     ███\n\
   ▗█████▖\n\
    "
    }

    pub fn notation<'a>() -> &'a str {
        "K"
    }

    pub fn mini_draw() -> &'static str {
        "♖"
    }
}
