mod chip;
use crate::chip::Chip;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn gpu_power() {
        let gpu = Chip {chip_type: String::from("nvidia"), hashes_s: 100.0, power:10.0};
        assert_eq!(gpu.watt_per_hash(), 0.1);
    }

    #[test]
    fn time_to_mine() {
        let gpu = Chip {chip_type: String::from("nvidia"), hashes_s: 100.0, power:10.0};
        assert_eq!(gpu.time_to_mine_one_bitcoin(), 1);
    }

    #[test]
    fn gtx_80() {
        let test_chip = Chip::gtx_80();
        assert_eq!(test_chip.chip_type, "gtx-80".to_string());
        assert_eq!(test_chip.hashes_s,1000.0);
        assert_eq!(test_chip.power, 10.0);
    }

    #[test]
    fn mining_rig_hashes() {
        let chips: Vec<Chip> = vec![Chip::gtx_80(), Chip::gtx_80()];
        let mining_rig = MiningRig::new(chips);
        assert_eq!(mining_rig.total_hash_capability, 2000.0);
    }
}

#[derive(Debug)]
//elements that are beyond control of mining rig setup
pub struct ExternalParameters {
    pub power_cost: f64,
    pub hashes_per_btc: f64,
    pub btc_price: f64,
}

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

