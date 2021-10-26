class Livestock(name: String) : Resource(name) {
    lateinit var livestockUses: List<LivestockUse>

    enum class LivestockUse {
        MEAT,
        DAIRY,
        EGGS,
        FIBER,
        LABOR,
        OTHER
    }
}