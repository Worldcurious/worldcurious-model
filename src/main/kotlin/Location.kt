abstract class Location(var name: String, var population: Int) {
    val economy: Economy = Economy(this)
}