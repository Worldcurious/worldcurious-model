import java.util.*

class NamedCharacter(var name: String, var isAlive: Boolean) {
    lateinit var gender: Gender
    lateinit var birthdate: Date
    lateinit var build: Build
    lateinit var weight: Weight
    lateinit var height: Height

    enum class Gender {
        MALE,
        FEMALE,
        OTHER
    }

    enum class Build {
        LIGHT,
        AVERAGE,
        STRONG
    }

    enum class Weight {
        SLIM,
        AVERAGE,
        HEAVY
    }

    enum class Height {
        SHORT,
        AVERAGE,
        TALL
    }
}