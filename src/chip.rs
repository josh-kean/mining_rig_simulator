#[derive(Debug)]
//specific to mining chip
pub struct Chip {
    pub chip_type: String,
    pub hashes_s: f64, //in 
    pub power: f64 //in watts
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
