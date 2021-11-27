pub const EMPTY_SVG_BG: &str = r###"
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
      <g fill="none">
      </g>
  </svg>
"###;

pub const NUMS_SVG_BG_ARRAY: [&str; 9] = [
    EMPTY_SVG_BG,
    include_str!("../assets/numbers/1.svg"),
    include_str!("../assets/numbers/2.svg"),
    include_str!("../assets/numbers/3.svg"),
    include_str!("../assets/numbers/4.svg"),
    include_str!("../assets/numbers/5.svg"),
    include_str!("../assets/numbers/6.svg"),
    include_str!("../assets/numbers/7.svg"),
    include_str!("../assets/numbers/8.svg"),
];

pub const TILE_OPENED_SVG_BG: &str = include_str!("../assets/tiles/opened.svg");
pub const TILE_UNOPENED_SVG_BG: &str = include_str!("../assets/tiles/unopened.svg");

pub const BOMB_SIGN_SVG_BG: &str = include_str!("../assets/signs/bomb.svg");
pub const FLAG_SIGN_SVG_BG: &str = include_str!("../assets/signs/flag.svg");
pub const QUESTION_MARK_SIGN_SVG_BG: &str = include_str!("../assets/signs/questionmark.svg");