mod economy;
mod government;

mod location {
    pub trait Location {
        // TODO
    }
    
    pub struct Faction {
        id: u32,
        name: String,
        population: u64,
        econ: economy::Economy<Faction>,
        gov: government::Government,
    }
    
    pub struct Region {
        id: u32,
        name: String,
        population: u64,
        econ: Economy<Region>,
    }
    
    pub struct City {
        id: u32,
        name: String,
        population: u64,
        econ: Economy<City>,
    }
}
