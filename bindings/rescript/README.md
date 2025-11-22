# Bunsenite Rescript Bindings

Type-safe Rescript bindings for [Bunsenite](https://gitlab.com/campaign-for-cooler-coding-and-programming/bunsenite) via C FFI.

## Installation

1. Build the Bunsenite native library:

```bash
cd ../..
cargo build --release
```

2. Add Bunsenite bindings to your Rescript project:

```bash
# Copy bindings to your project
cp bindings/rescript/Bunsenite.res src/
```

3. Configure FFI in your `bsconfig.json`:

```json
{
  "name": "your-project",
  "sources": [
    {
      "dir": "src",
      "subdirs": true
    }
  ],
  "bs-dependencies": [],
  "external-stdlibs": ["bunsenite"]
}
```

## Usage

### Basic Parsing

```rescript
open Bunsenite

let config = parseNickel(
  "{
    name = \"my-app\",
    version = \"1.0.0\",
    port = 8080,
  }",
  "config.ncl"
)

switch config {
| Ok(json) => Js.log(json)
| Error(err) => Js.log2("Error:", errorToString(err))
}
```

### Parse File

```rescript
open Bunsenite

let config = parseFile("./config.ncl")

switch config {
| Ok(json) => {
    // Access nested values
    let port = getConfigValue(json, list{"server", "port"})
    Js.log2("Server port:", port)
  }
| Error(err) => Js.log2("Error:", errorToString(err))
}
```

### Validation

```rescript
open Bunsenite

let result = validateNickel("{foo = 42}", "config.ncl")

switch result {
| Ok() => Js.log("Valid!")
| Error(err) => Js.log2("Invalid:", errorToString(err))
}
```

### Library Info

```rescript
open Bunsenite

Js.log2("Version:", getVersion())
Js.log2("RSR Tier:", getRSRTier())
Js.log2("TPCF Perimeter:", getTPCFPerimeter())
```

## API Reference

### Types

```rescript
type result<'a, 'e> = Ok('a) | Error('e)

type error =
  | ParseError(string)
  | ValidationError(string)
  | InvalidInput(string)

type parseResult = result<Js.Json.t, error>
type validateResult = result<unit, error>
```

### Functions

#### `parseNickel(source: string, name: string): parseResult`

Parse and evaluate a Nickel configuration string.

- `source`: The Nickel configuration source code
- `name`: A name for this configuration (used in error messages)
- Returns: `Ok(Js.Json.t)` on success, `Error(error)` on failure

#### `validateNickel(source: string, name: string): validateResult`

Validate a Nickel configuration without evaluating it.

- `source`: The Nickel configuration source code
- `name`: A name for this configuration (used in error messages)
- Returns: `Ok()` if valid, `Error(error)` if invalid

#### `parseFile(path: string): parseResult`

Parse a Nickel configuration file.

- `path`: Path to the Nickel configuration file
- Returns: `Ok(Js.Json.t)` on success, `Error(error)` on failure

#### `validateFile(path: string): validateResult`

Validate a Nickel configuration file.

- `path`: Path to the Nickel configuration file
- Returns: `Ok()` if valid, `Error(error)` if invalid

#### `getVersion(): string`

Get Bunsenite library version.

- Returns: Version string (e.g., "0.1.0")

#### `getRSRTier(): string`

Get RSR compliance tier.

- Returns: RSR tier (e.g., "bronze")

#### `getTPCFPerimeter(): int`

Get TPCF perimeter number.

- Returns: Perimeter number (3 for Community Sandbox)

### Helper Functions

#### `getConfigValue(json: Js.Json.t, path: list<string>): option<Js.Json.t>`

Get a value from a configuration object by key path.

Example:
```rescript
let port = getConfigValue(config, list{"server", "port"})
```

#### `errorToString(err: error): string`

Convert an error to a string for display.

## Architecture

```
┌─────────────────┐
│  Rescript       │
│  (Type-safe)    │
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

## Type Safety

Rescript provides:
- **Compile-time type checking**: Catch errors before runtime
- **Sound type system**: No `null` or `undefined` surprises
- **Pattern matching**: Exhaustive error handling via `result` type
- **Immutability**: Default immutability prevents bugs

Combined with Bunsenite's Rust core:
- **Memory safety**: Rust ownership model
- **Type safety**: Nickel + Rust type checking
- **No runtime errors**: Caught at compile time

## Security

- **Memory Safety**: Rust ownership model prevents memory errors
- **Type Safety**: Rescript + Nickel + Rust triple type checking
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
