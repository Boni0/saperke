use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use druid::{Data, Lens};
use druid::widget::SvgData;

const NUMBERS_DIR: &str = "numbers";

const SIGNS_DIR: &str = "signs";
const SIGNS_VARIANTS: [&str; 3] = [
    "bomb",
    "flag",
    "questionmark"
];

const TILES_DIR: &str = "tiles";
const TILES_VARIANTS: [&str; 2] = [
    "opened",
    "unopened"
];

#[derive(Clone, Data, Lens)]
pub struct SvgAssets {
    pub hash_map: Rc<HashMap<String, SvgData>>
}

fn get_svg_data (paths: &[&str]) -> SvgData {
    let mut read_path = String::new();

    for path in paths {
        read_path.push_str(format!("/{}", *path).as_str());
    }

    fs::read_to_string(format!("./src/assets{}.svg", read_path).as_str())
        .unwrap_or_default()
        .parse::<SvgData>()
        .unwrap_or(SvgData::default())
}

impl SvgAssets {
    pub fn init() -> Self {
        let mut hash_map: Rc<HashMap<String, SvgData>>  = Rc::new(HashMap::new());

        for number in 1..=8 {
            let key = number.to_string();
            Rc::get_mut(&mut hash_map).unwrap().insert(
                key.clone(), 
                get_svg_data(&[NUMBERS_DIR, key.as_str()])
            );
        }

        for sign_p in SIGNS_VARIANTS.iter() {
            let sign: &str = *sign_p;
            Rc::get_mut(&mut hash_map).unwrap().insert(
                sign.to_string(), 
                get_svg_data(&[SIGNS_DIR, sign])
            );
        }

        for tile_p in TILES_VARIANTS.iter() {
            let tile: &str = *tile_p;
            Rc::get_mut(&mut hash_map).unwrap().insert(
                tile.to_string(), 
                get_svg_data(&[TILES_DIR, tile])
            );
        }

        Self { hash_map }
    }

    pub fn get (&self, name: &str) -> SvgData {
        self
            .hash_map
            .get(name)
            .and_then(|svg_ref| {
                Some(Clone::clone(svg_ref))
            })
            .unwrap_or(SvgData::default())
    }
}