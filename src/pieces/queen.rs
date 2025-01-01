pub struct Queen;

impl Queen {
    pub fn draw() -> &'static str {
        "\
    \n\
◀█▟█▙█▶\n\
  ◥█◈█◤\n\
  ███\n\
▗█████▖\n\
    "
    }

    pub fn notation<'a>() -> &'a str {
        "Q"
    }

    pub fn mini_draw() -> &'static str {
        "♕"
    }
}
