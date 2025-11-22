# Example Bunsenite Configuration
# This demonstrates various Nickel features

{
  # Application metadata
  name = "example-app",
  version = "1.0.0",
  description = "An example application configuration",

  # Server configuration
  server = {
    host = "0.0.0.0",
    port = 8080,
    workers = 4,
    timeout_seconds = 30,
  },

  # Database configuration
  database = {
    url = "postgres://localhost:5432/mydb",
    max_connections = 20,
    timeout_ms = 5000,
  },

  # Feature flags
  features = {
    enable_logging = true,
    enable_metrics = true,
    enable_tracing = false,
    debug_mode = false,
  },

  # Computed values
  full_name = name ++ " v" ++ version,
  server_url = "http://" ++ server.host ++ ":" ++ std.string.from_number server.port,

  # List example
  allowed_origins = [
    "http://localhost:3000",
    "http://localhost:8080",
    "https://example.com",
  ],

  # Nested configuration
  logging = {
    level = "info",
    format = "json",
    outputs = ["stdout", "file"],
    file_path = "/var/log/app.log",
  },
}
