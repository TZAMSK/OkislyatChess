pub struct Bishop;

impl Bishop {
    pub fn draw() -> &'static str {
        "\
    \n\
       ⭘\n\
      █✝█\n\
      ███\n\
    ▗█████▖\n\
    "
    }

    pub fn notation<'a>() -> &'a str {
        "B"
    }

    pub fn mini_draw() -> &'static str {
        "♗"
    }
}
