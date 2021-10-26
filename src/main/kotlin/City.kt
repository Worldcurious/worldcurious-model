class City(name: String, population: Int) : Location(name, population) {
    lateinit var region: Region
}
