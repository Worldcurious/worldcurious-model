mod trade;

pub trait Resource {
    // TODO
}

pub struct BaseResource {
    id: u32,
    name: String,

    importMarkets: Vec<trade::TradeExchange>,
    exportMarkets: Vec<trade::TradeExchange>,

    economies: Vec<economy::Economy>,
}

pub mod labor {
    pub struct Labor {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,
    
        laborType: LaborType,
    }
    
    pub enum LaborType {
        Manual,
        Unskilled,
        Skilled,
        Educated,
    }
}


pub struct Component {
    id: u32,
    name: String,

    importMarkets: Vec<trade::TradeExchange>,
    exportMarkets: Vec<trade::TradeExchange>,
}

pub struct CapitalGood {
    id: u32,
    name: String,

    importMarkets: Vec<trade::TradeExchange>,
    exportMarkets: Vec<trade::TradeExchange>,
}

pub mod crop {
    pub struct Crop {
        id: u32,
        name: String,
    
        importMarkets: Vec<trade::TradeExchange>,
        exportMarkets: Vec<trade::TradeExchange>,
    
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
