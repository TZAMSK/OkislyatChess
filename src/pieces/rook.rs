pub struct Rook;

impl Rook {
    pub fn draw() -> &'static str {
        "\
    \n\
    █▟█▙█\n\
    ▜███▛\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
    }

    pub fn notation<'a>() -> &'a str {
        "R"
    }

    pub fn mini_draw() -> &'static str {
        "♖"
    }
}
