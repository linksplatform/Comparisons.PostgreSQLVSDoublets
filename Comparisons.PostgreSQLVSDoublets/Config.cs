namespace Comparisons.PostgreSQLVSDoublets
{
    public static class Config
    {
        public static string HasuraEndpoint { get; set; } = "http://localhost:8080/v1/graphql";
        public static string HasuraAdminSecret { get; set; } = "myadminsecretkey";
        public static string DoubletsEndpoint { get; set; } = "http://localhost:5000/graphql";
    }
}
