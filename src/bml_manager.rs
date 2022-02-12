use bevy_bulletml::{parse::BulletMLParser, BulletML};

trait BMLProvider {
    fn get_bml(&self) -> &BulletML;
}

pub struct BMLManager {
    pub bml: BulletML,
}

impl BMLManager {
    pub fn new(bml_file_path: String) -> Self {
        let bml = BulletMLParser::with_capacities(0, 128)
            .parse_file(bml_file_path)
            .unwrap();
        Self { bml }
    }
}
