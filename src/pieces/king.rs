pub struct King;

impl King {
    pub fn draw() -> &'static str {
        "\
            \n\
      ✚\n\
    ▞▀▄▀▚\n\
   ▐▙▄█▄▟▌\n\
    ▐███▌\n\
     ███\n\
  ▗███████▖\n\
    "
    }

    pub fn notation<'a>() -> &'a str {
        "K"
    }

    pub fn mini_draw() -> &'static str {
        "♔"
    }
}
