pub use chip::Chip;

mod MiningRig{
#[derive(Debug)]
pub struct MiningRig {
    pub chip_collection: Vec<Chip>,
    total_hash_capability: f64,
    total_power_usage: f64,
}

impl MiningRig {
    fn calc_hash_capability(chips: &Vec<Chip>) -> f64 {
        chips.iter().map(|x| x.hashes_s).sum()
    }

    fn calc_power_usage(chips: &Vec<Chip>) -> f64 {
        chips.iter().map(|x| x.power).sum()
    }

    pub fn new(chips: Vec<Chip>) -> MiningRig {
        let total_hash_capability = MiningRig::calc_hash_capability(&chips);
        let total_power_usage = MiningRig::calc_power_usage(&chips);
        MiningRig{
            chip_collection: chips, 
            total_hash_capability: total_hash_capability,
            total_power_usage: total_power_usage,
        }
    }
}
}
