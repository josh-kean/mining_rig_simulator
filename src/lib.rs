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
//specific to mining chip
pub struct Chip {
    pub chip_type: String,
    pub hashes_s: f64, //in 
    pub power: f64 //in watts
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

impl Chip {
    pub fn gtx_80() -> Chip {
        Chip{chip_type: "gtx-80".to_string(), hashes_s: 1000.0, power: 10.0}
    }
    //dummy equation to calculate the cost per hash
    // want to calculate the watts per hash per second
    pub fn watt_per_hash(&self) -> f64 {
        self.power/self.hashes_s
    }

    pub fn cost_per_hash(&self, e_cost: f64) -> f64 {
        let wats_per_hash = self.watt_per_hash();
        //know the cost of a watt per hash
        //e_cost is the price per killowat hour
        //multiply wats_per_hash by 3600 to convert watts_per_hash into kwhr
        wats_per_hash*3600.0*e_cost //results in the cost per kwhr
    }

    pub fn time_to_mine_one_bitcoin(&self) -> usize {
        //returns the amount of time expected to mine a single bitcoin in seconds
        let current_hashes_per_bitcoin: f64 = 100.0; //100 for now, will be converted to a global unit
        (current_hashes_per_bitcoin/self.hashes_s) as usize
    }
}
