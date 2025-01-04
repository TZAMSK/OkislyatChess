use ratatui::style::Color;

pub const BOARD_WHITE: Color = Color::Rgb(231, 193, 147);
pub const BOARD_BLACK: Color = Color::Rgb(138, 95, 73);
pub const PIECE_WHITE: Color = Color::Rgb(251, 242, 230);
pub const PIECE_BLACK: Color = Color::Rgb(27, 15, 12);

pub const TITLE: &str = r"
 ██████╗ ██╗  ██╗██╗███████╗██╗   ██╗   ██╗ █████╗ ████████╗
██╔═══██╗██║ ██╔╝██║██╔════╝██║   ╚██╗ ██╔╝██╔══██╗╚══██╔══╝
██║   ██║█████╔╝ ██║███████╗██║    ╚████╔╝ ███████║   ██║   
██║   ██║██╔═██╗ ██║╚════██║██║     ╚██╔╝  ██╔══██║   ██║   
╚██████╔╝██║  ██╗██║███████║███████╗ ██║   ██║  ██║   ██║   
 ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝╚══════╝ ╚═╝   ╚═╝  ╚═╝   ╚═╝ 
";

pub const HELP: &str = r"
██╗  ██╗███████╗██╗     ██████╗
██║  ██║██╔════╝██║     ██╔══██╗
███████║█████╗  ██║     ██████╔╝
██╔══██║██╔══╝  ██║     ██╔═══╝
██║  ██║███████╗███████╗██║
╚═╝  ╚═╝╚══════╝╚══════╝╚═╝
";

pub const MENU_ITEMS: [&'static str; 4] = ["2 Players", "Bot", "Help", "Quit"];

pub const BOARD_LETTERS: [&'static str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

pub const PIECES: [&'static str; 6] = ["Pawn", "Rook", "Knight", "Bishop", "Queen", "King"];
