class Faction(name: String, population: Int) : Location(name, population){
    lateinit var regions: List<Region>
    lateinit var capital: City
    lateinit var cities: List<City>
    lateinit var allies: List<Faction>
    lateinit var rivals: List<Faction>
    lateinit var tradePartners: List<Faction>
    lateinit var government: Government
}
