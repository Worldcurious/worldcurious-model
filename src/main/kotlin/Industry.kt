open class Industry(var name: String) {
    lateinit var economies: List<Economy>
    lateinit var complementaryInputs: List<Resource>
    lateinit var substituteInputSets: List<SubstituteResourceSet>
    lateinit var outputs: List<Resource>

    class SubstituteResourceSet {
        val id: Int = 0 // TODO

    }
}
