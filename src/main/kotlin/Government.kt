class Government(val faction: Faction) {
    val id: Int = 0 // TODO
    lateinit var powerStructure: PowerStructure
    lateinit var powerSource: PowerSource
    lateinit var powerIdeologies: List<PowerIdeology>
    lateinit var officials: HashMap<String, NamedCharacter>

    enum class PowerStructure {
        ANARCHY,
        CONFEDERATION,
        FEDERATION,
        UNITARY_STATE
    }

    enum class PowerSource {
        AUTOCRACY,
        DEMOCRACY,
        OLIGARCHY
    }

    enum class PowerIdeology {
        CONSTITUTIONAL,
        MONARCHY,
        REPUBLIC
    }
}
