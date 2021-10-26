class Region(name: String, population: Int) : Location(name, population) {
    lateinit var capital: City
    lateinit var cities: List<City>
    lateinit var faction: Faction
}
