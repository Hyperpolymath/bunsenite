#!/usr/bin/env deno run --allow-ffi --allow-read

// Bunsenite Deno Example
// Demonstrates how to use Bunsenite from Deno

import {
  getTPCFPerimeter,
  getRSRTier,
  getVersion,
  parseFile,
  parseNickel,
  validateNickel,
} from "./bunsenite.ts";

console.log("=== Bunsenite Deno Example ===\n");

// Show library info
console.log("Library Information:");
console.log(`  Version: ${getVersion()}`);
console.log(`  RSR Tier: ${getRSRTier()}`);
console.log(`  TPCF Perimeter: ${getTPCFPerimeter()}`);
console.log("");

// Example 1: Parse simple inline config
console.log("Example 1: Parse inline config");
const config1 = parseNickel(
  `{
    name = "deno-example",
    version = "1.0.0",
    port = 8080,
  }`,
  "inline.ncl",
);
console.log("Result:", JSON.stringify(config1, null, 2));
console.log("");

// Example 2: Parse with computations
console.log("Example 2: Parse with computations");
const config2 = parseNickel(
  `{
    base_port = 8000,
    api_port = base_port + 80,
    db_port = base_port + 432,
    url = "http://localhost:" ++ std.string.from_number api_port,
  }`,
  "computed.ncl",
);
console.log("Result:", JSON.stringify(config2, null, 2));
console.log("");

// Example 3: Validate config
console.log("Example 3: Validate config");
try {
  validateNickel('{ valid = true, works = "yes" }', "valid.ncl");
  console.log("✓ Config is valid");
} catch (e) {
  console.error("✗ Config is invalid:", e.message);
}
console.log("");

// Example 4: Validate invalid config (should fail)
console.log("Example 4: Validate invalid config");
try {
  validateNickel("{ invalid = }", "invalid.ncl"); // Missing value
  console.log("✓ Config is valid");
} catch (e) {
  console.log("✓ Correctly detected invalid config");
}
console.log("");

// Example 5: Parse file (if it exists)
console.log("Example 5: Parse file");
try {
  const config = await parseFile("../../examples/config.ncl");
  console.log("Parsed config from file:");
  console.log(`  Name: ${(config as any).name}`);
  console.log(`  Version: ${(config as any).version}`);
  console.log(`  Server port: ${(config as any).server.port}`);
} catch (e) {
  console.log(`Could not parse file: ${e.message}`);
  console.log("(This is expected if bunsenite hasn't been built yet)");
}
console.log("");

// Example 6: Advanced features
console.log("Example 6: Advanced features");
const config6 = parseNickel(
  `{
    # Comments work!
    app_name = "bunsenite",

    # Lists
    allowed_hosts = ["localhost", "127.0.0.1", "::1"],

    # Nested records
    database = {
      host = "localhost",
      port = 5432,
      max_connections = 20,
    },

    # Computed values
    db_url = "postgres://" ++ database.host ++ ":" ++ std.string.from_number database.port,
  }`,
  "advanced.ncl",
);
console.log("Advanced config:", JSON.stringify(config6, null, 2));

console.log("\n✓ All examples completed successfully!");
