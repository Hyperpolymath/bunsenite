// Bunsenite Deno FFI Bindings
// TypeScript bindings for Deno runtime using native FFI
//
// NOTE: This is Deno-specific TypeScript, NOT plain TypeScript!
// It uses Deno.dlopen for native FFI calls to the Zig C ABI layer.
//
// Usage:
//   import { parseNickel, validateNickel } from "./bunsenite.ts";
//   const result = parseNickel('{ foo = 42 }', "config.ncl");
//   console.log(result);

// Detect library path based on platform
function getLibraryPath(): string {
  const platform = Deno.build.os;
  const libName = platform === "windows" ? "bunsenite.dll"
    : platform === "darwin" ? "libbunsenite.dylib"
    : "libbunsenite.so";

  // Try common locations
  const paths = [
    `../../target/release/${libName}`,
    `./target/release/${libName}`,
    `./${libName}`,
  ];

  for (const path of paths) {
    try {
      Deno.statSync(path);
      return path;
    } catch {
      // File doesn't exist, try next
    }
  }

  throw new Error(
    `Could not find ${libName}. Please build with: cargo build --release`,
  );
}

// FFI symbol definitions
// These match the C ABI exported by the Zig layer
const symbols = {
  // Parse Nickel string to JSON
  // char* parse_nickel(const char* source, const char* name)
  parse_nickel: {
    parameters: ["pointer", "pointer"],
    result: "pointer",
  },

  // Validate Nickel without evaluating
  // int validate_nickel(const char* source, const char* name)
  validate_nickel: {
    parameters: ["pointer", "pointer"],
    result: "i32",
  },

  // Free string allocated by Rust
  // void free_string(char* ptr)
  free_string: {
    parameters: ["pointer"],
    result: "void",
  },

  // Get library version
  // const char* version()
  version: {
    parameters: [],
    result: "pointer",
  },

  // Get RSR tier
  // const char* rsr_tier()
  rsr_tier: {
    parameters: [],
    result: "pointer",
  },

  // Get TPCF perimeter
  // uint8_t tpcf_perimeter()
  tpcf_perimeter: {
    parameters: [],
    result: "u8",
  },
} as const;

// Load the native library
let lib: Deno.DynamicLibrary<typeof symbols> | null = null;

function getLib(): Deno.DynamicLibrary<typeof symbols> {
  if (!lib) {
    const libPath = getLibraryPath();
    lib = Deno.dlopen(libPath, symbols);
  }
  return lib;
}

// Helper: Convert JS string to C string (null-terminated)
function toCString(str: string): Uint8Array {
  const encoder = new TextEncoder();
  const encoded = encoder.encode(str + "\0");
  return encoded;
}

// Helper: Convert C string pointer to JS string
function fromCString(ptr: Deno.UnsafePointer): string {
  if (!ptr) {
    throw new Error("Null pointer received from C");
  }
  const view = new Deno.UnsafePointerView(ptr);
  return view.getCString();
}

/**
 * Parse and evaluate a Nickel configuration string
 *
 * @param source - The Nickel configuration source code
 * @param name - A name for this configuration (used in error messages)
 * @returns Parsed configuration as a JavaScript object
 * @throws Error if parsing or evaluation fails
 *
 * @example
 * ```typescript
 * const config = parseNickel('{ name = "example", port = 8080 }', "config.ncl");
 * console.log(config.port); // 8080
 * ```
 */
export function parseNickel(source: string, name: string): unknown {
  const library = getLib();

  const sourceBytes = toCString(source);
  const nameBytes = toCString(name);

  const resultPtr = library.symbols.parse_nickel(
    sourceBytes,
    nameBytes,
  ) as Deno.UnsafePointer;

  if (!resultPtr) {
    throw new Error(`Failed to parse Nickel config: ${name}`);
  }

  try {
    const jsonString = fromCString(resultPtr);
    return JSON.parse(jsonString);
  } finally {
    // Free the string allocated by Rust
    library.symbols.free_string(resultPtr);
  }
}

/**
 * Validate a Nickel configuration without evaluating it
 *
 * @param source - The Nickel configuration source code
 * @param name - A name for this configuration (used in error messages)
 * @returns true if valid, throws Error if invalid
 * @throws Error if validation fails
 *
 * @example
 * ```typescript
 * try {
 *   validateNickel('{ foo = 42 }', "config.ncl");
 *   console.log("Valid!");
 * } catch (e) {
 *   console.error("Invalid:", e.message);
 * }
 * ```
 */
export function validateNickel(source: string, name: string): boolean {
  const library = getLib();

  const sourceBytes = toCString(source);
  const nameBytes = toCString(name);

  const result = library.symbols.validate_nickel(
    sourceBytes,
    nameBytes,
  );

  if (result !== 0) {
    throw new Error(`Validation failed for: ${name}`);
  }

  return true;
}

/**
 * Get Bunsenite library version
 *
 * @returns Version string (e.g., "0.1.0")
 *
 * @example
 * ```typescript
 * console.log("Bunsenite version:", getVersion());
 * ```
 */
export function getVersion(): string {
  const library = getLib();
  const ptr = library.symbols.version() as Deno.UnsafePointer;
  return fromCString(ptr);
}

/**
 * Get RSR compliance tier
 *
 * @returns RSR tier (e.g., "bronze")
 *
 * @example
 * ```typescript
 * console.log("RSR tier:", getRSRTier());
 * ```
 */
export function getRSRTier(): string {
  const library = getLib();
  const ptr = library.symbols.rsr_tier() as Deno.UnsafePointer;
  return fromCString(ptr);
}

/**
 * Get TPCF perimeter number
 *
 * @returns Perimeter number (3 for Community Sandbox)
 *
 * @example
 * ```typescript
 * console.log("TPCF perimeter:", getTPCFPerimeter());
 * ```
 */
export function getTPCFPerimeter(): number {
  const library = getLib();
  return library.symbols.tpcf_perimeter();
}

/**
 * Parse a Nickel configuration file
 *
 * @param path - Path to the Nickel configuration file
 * @returns Parsed configuration as a JavaScript object
 * @throws Error if file cannot be read or parsing fails
 *
 * @example
 * ```typescript
 * const config = await parseFile("./config.ncl");
 * console.log(config);
 * ```
 */
export async function parseFile(path: string): Promise<unknown> {
  const source = await Deno.readTextFile(path);
  return parseNickel(source, path);
}

/**
 * Validate a Nickel configuration file
 *
 * @param path - Path to the Nickel configuration file
 * @returns true if valid, throws Error if invalid
 * @throws Error if file cannot be read or validation fails
 *
 * @example
 * ```typescript
 * try {
 *   await validateFile("./config.ncl");
 *   console.log("File is valid!");
 * } catch (e) {
 *   console.error("Invalid file:", e.message);
 * }
 * ```
 */
export async function validateFile(path: string): Promise<boolean> {
  const source = await Deno.readTextFile(path);
  return validateNickel(source, path);
}

// Cleanup on exit
globalThis.addEventListener("unload", () => {
  if (lib) {
    lib.close();
    lib = null;
  }
});

// Export type definitions
export type BunseniteConfig = Record<string, unknown>;

// Re-export for convenience
export default {
  parseNickel,
  validateNickel,
  parseFile,
  validateFile,
  getVersion,
  getRSRTier,
  getTPCFPerimeter,
};
