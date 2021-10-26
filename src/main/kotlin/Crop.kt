class Crop(name: String) : Resource(name) {
    lateinit var cropType: CropType

    enum class CropType {
        FOOD,
        CASH,
        FOOD_AND_CASH
    }
}