class Labor(name: String) : Resource(name) {
    lateinit var laborType: LaborType

    enum class LaborType {
        MANUAL,
        UNSKILLED,
        SKILLED,
        EDUCATED
    }
}