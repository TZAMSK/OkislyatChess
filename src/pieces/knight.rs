pub struct Knight;

impl Knight {
    pub fn draw() -> &'static str {
        "\
            \n\
    \n\
    ▟█▛██▙\n\
   ▟██████\n\
   ▀▀▀▐██▌\n\
    ▟███▌\n\
   ▟██████\n\
    "
    }

    pub fn notation<'a>() -> &'a str {
        "N"
    }

    pub fn mini_draw() -> &'static str {
        "♘"
    }
}
