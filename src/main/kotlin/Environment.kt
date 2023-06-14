object Environment {
    private const val CUSTOM_FORK_OWNER_ENV = "KMC_FRK_OWNER"
    private const val CUSTOM_TOOL_ENV = "KMC_CRTD_WITH"
    private const val CUSTOM_TOOL_URL_ENV = "KMC_CRTD_WITH_URL"
    private const val CI_ENV = "CI"

    val isCI = System.getenv(CI_ENV)?.toBooleanStrictOrNull() == true
    val forkOverride: String? = System.getenv(CUSTOM_FORK_OWNER_ENV)
    val customToolName: String? = System.getenv(CUSTOM_TOOL_ENV)
    val customToolURL: String? = System.getenv(CUSTOM_TOOL_URL_ENV)
}
