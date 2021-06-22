mod trade;
mod industry;

pub trait Resource {
    // TODO
}

pub mod base_resource {
    pub struct BaseResource {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,

        producers: Vec<industry::Industry>,
        consumers: Vec<industry::Industry>,
    
        economies: Vec<economy::Economy>,
    }
}

pub mod labor {
    pub struct Labor {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,
    
        producers: Vec<industry::Industry>,
        consumers: Vec<industry::Industry>,

        laborType: LaborType,
    }
    
    pub enum LaborType {
        Manual,
        Unskilled,
        Skilled,
        Educated,
    }
}

pub mod component {
    pub struct Component {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,

        producers: Vec<industry::Industry>,
        consumers: Vec<industry::Industry>,
    }
}

pub mod capital_good {
    pub struct CapitalGood {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,

        producers: Vec<industry::Industry>,
        consumers: Vec<industry::Industry>,
    }
}

pub mod crop {
    pub struct Crop {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,

        producers: Vec<industry::Industry>,
        consumers: Vec<industry::Industry>,
    
        cropType: CropType,
        kcalPerAcre: u32,
    }
    
    pub enum CropType {
        Food,
        Cash,
        FoodAndCash,
    }
}

pub mod livestock {
    pub struct Livestock {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,

        producers: Vec<industry::Industry>,
        consumers: Vec<industry::Industry>,

        uses: Vec<LivestockUse>,
    }
    
    pub enum LivestockUse {
        Meat,
        Dairy,
        Eggs,
        Fiber,
        Labor,
        Other,
    }
}
