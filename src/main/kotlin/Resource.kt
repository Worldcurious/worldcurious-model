abstract class Resource(var name: String) {
    lateinit var consumers: List<Industry>
    lateinit var importMarkets: List<TradeExchange>
    lateinit var exportMarkets: List<TradeExchange>
    lateinit var substituteSets: List<Industry.SubstituteResourceSet>
}
