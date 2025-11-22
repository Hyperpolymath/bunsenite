# Bunsenite Deno Bindings

TypeScript bindings for [Bunsenite](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite) using Deno's native FFI.

## Installation

1. Build the Bunsenite native library:

```bash
cd ../..
cargo build --release
```

2. Import the bindings in your Deno code:

```typescript
import { parseNickel } from "https://raw.githubusercontent.com/example/bunsenite/main/bindings/deno/bunsenite.ts";
```

Or use local path:

```typescript
import { parseNickel } from "./bunsenite.ts";
```

## Usage

### Basic Parsing

```typescript
import { parseNickel } from "./bunsenite.ts";

const config = parseNickel(
  `{
    name = "my-app",
    version = "1.0.0",
    port = 8080,
  }`,
  "config.ncl"
);

console.log(config.port); // 8080
```

### Parse File

```typescript
import { parseFile } from "./bunsenite.ts";

const config = await parseFile("./config.ncl");
console.log(config);
```

### Validation

```typescript
import { validateNickel } from "./bunsenite.ts";

try {
  validateNickel('{ foo = 42 }', "config.ncl");
  console.log("Valid!");
} catch (e) {
  console.error("Invalid:", e.message);
}
```

### Library Info

```typescript
import { getVersion, getRSRTier, getTPCFPerimeter } from "./bunsenite.ts";

console.log("Version:", getVersion());
console.log("RSR Tier:", getRSRTier());
console.log("TPCF Perimeter:", getTPCFPerimeter());
```

## API Reference

### `parseNickel(source: string, name: string): unknown`

Parse and evaluate a Nickel configuration string.

- `source`: The Nickel configuration source code
- `name`: A name for this configuration (used in error messages)
- Returns: Parsed configuration as a JavaScript object
- Throws: Error if parsing or evaluation fails

### `validateNickel(source: string, name: string): boolean`

Validate a Nickel configuration without evaluating it.

- `source`: The Nickel configuration source code
- `name`: A name for this configuration (used in error messages)
- Returns: `true` if valid
- Throws: Error if validation fails

### `parseFile(path: string): Promise<unknown>`

Parse a Nickel configuration file.

- `path`: Path to the Nickel configuration file
- Returns: Parsed configuration as a JavaScript object
- Throws: Error if file cannot be read or parsing fails

### `validateFile(path: string): Promise<boolean>`

Validate a Nickel configuration file.

- `path`: Path to the Nickel configuration file
- Returns: `true` if valid
- Throws: Error if file cannot be read or validation fails

### `getVersion(): string`

Get Bunsenite library version.

- Returns: Version string (e.g., "0.1.0")

### `getRSRTier(): string`

Get RSR compliance tier.

- Returns: RSR tier (e.g., "bronze")

### `getTPCFPerimeter(): number`

Get TPCF perimeter number.

- Returns: Perimeter number (3 for Community Sandbox)

## Permissions

Deno requires the following permissions:

- `--allow-ffi`: To load the native library
- `--allow-read`: To read configuration files (if using `parseFile`)

Example:

```bash
deno run --allow-ffi --allow-read example.ts
```

## Examples

See [example.ts](./example.ts) for comprehensive examples.

Run the example:

```bash
# Make sure bunsenite is built first
cd ../..
cargo build --release

# Run example
cd bindings/deno
deno run --allow-ffi --allow-read example.ts
```

## Platform Support

| Platform | Library Name        | Status |
| -------- | ------------------- | ------ |
| Linux    | `libbunsenite.so`   | ✅      |
| macOS    | `libbunsenite.dylib`| ✅      |
| Windows  | `bunsenite.dll`     | ✅      |

The bindings automatically detect your platform and load the correct library.

## Architecture

```
┌─────────────────┐
│  Deno Runtime   │
│  (TypeScript)   │
└────────┬────────┘
         │ FFI
         ▼
  ┌──────────┐
  │ Zig FFI  │
  │ (C ABI)  │
  └─────┬────┘
        │
        ▼
┌─────────────────┐
│   Rust Core     │
│   (lib.rs)      │
│                 │
│ nickel-lang-core│
│     0.9.1       │
└─────────────────┘
```

## Performance

~90% of native Rust performance (minimal C ABI overhead).

## Security

- **Memory Safety**: Rust ownership model prevents memory errors
- **Type Safety**: Full type checking via Nickel + Rust
- **No `unsafe`**: Zero unsafe code blocks in Bunsenite core
- **Offline-First**: No network dependencies

## License

Dual MIT + Palimpsest License v0.8

See [LICENSE](../../LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development guidelines.

## Support

- **Issues**: [GitLab Issues](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues)
- **Discussions**: [GitLab Discussions](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite/-/issues)
- **Documentation**: [Main README](../../README.md)

---

Made with ❤️ by the Campaign for Cooler Coding and Programming
