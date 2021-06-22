mod economy;
mod government;
mod trade;

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

        capital: City,
        
        rivals: Vec<Faction>,
        allies: Vec<Faction>,
        tradePartners: Vec<Faction>,

        regions: Vec<Region>,
        cities: Vec<City>,
    }
    
    pub struct Region {
        id: u32,
        name: String,
        population: u64,
        
        econ: Economy<Region>,

        faction: Faction,
        capital: City,

        cities: Vec<City>,
    }
    
    pub struct City {
        id: u32,
        name: String,
        population: u64,

        econ: Economy<City>,

        faction: Faction,
        region: Region,

        markets: Vec<trade::TradeExchange>,

    }
}
