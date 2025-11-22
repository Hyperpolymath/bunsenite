// Bunsenite Rescript Bindings
// Type-safe Rescript bindings for Bunsenite via C FFI
//
// Usage:
//   open Bunsenite
//   let config = parseNickel("{foo = 42}", "config.ncl")
//   Js.log(config)

// External C FFI declarations
// These bind to the C ABI provided by the Zig layer

@module("./bunsenite_ffi")
external parseNickelRaw: (string, string) => Js.Nullable.t<string> = "parse_nickel"

@module("./bunsenite_ffi")
external validateNickelRaw: (string, string) => int = "validate_nickel"

@module("./bunsenite_ffi")
external versionRaw: unit => string = "version"

@module("./bunsenite_ffi")
external rsrTierRaw: unit => string = "rsr_tier"

@module("./bunsenite_ffi")
external tpcfPerimeterRaw: unit => int = "tpcf_perimeter"

// Result type for error handling
type result<'a, 'e> = Ok('a) | Error('e)

// Error type
type error =
  | ParseError(string)
  | ValidationError(string)
  | InvalidInput(string)

// Parse and evaluate a Nickel configuration string
//
// Example:
//   let config = parseNickel("{name = \"example\", port = 8080}", "config.ncl")
//   switch config {
//   | Ok(json) => Js.log(json)
//   | Error(err) => Js.log2("Error:", err)
//   }
let parseNickel = (source: string, name: string): result<Js.Json.t, error> => {
  let result = parseNickelRaw(source, name)

  switch Js.Nullable.toOption(result) {
  | Some(jsonString) =>
    try {
      let parsed = Js.Json.parseExn(jsonString)
      Ok(parsed)
    } catch {
    | _ => Error(ParseError("Failed to parse JSON result"))
    }
  | None => Error(ParseError("Failed to parse Nickel configuration: " ++ name))
  }
}

// Validate a Nickel configuration without evaluating it
//
// Example:
//   let result = validateNickel("{foo = 42}", "config.ncl")
//   switch result {
//   | Ok() => Js.log("Valid!")
//   | Error(err) => Js.log2("Invalid:", err)
//   }
let validateNickel = (source: string, name: string): result<unit, error> => {
  let resultCode = validateNickelRaw(source, name)

  if resultCode == 0 {
    Ok()
  } else {
    Error(ValidationError("Validation failed for: " ++ name))
  }
}

// Get library version
//
// Example:
//   let ver = getVersion()
//   Js.log2("Version:", ver)
let getVersion = (): string => {
  versionRaw()
}

// Get RSR compliance tier
//
// Example:
//   let tier = getRSRTier()
//   Js.log2("RSR Tier:", tier)
let getRSRTier = (): string => {
  rsrTierRaw()
}

// Get TPCF perimeter number
//
// Example:
//   let perimeter = getTPCFPerimeter()
//   Js.log2("TPCF Perimeter:", perimeter)
let getTPCFPerimeter = (): int => {
  tpcfPerimeterRaw()
}

// Helper: Parse Nickel file from filesystem
// Requires Node.js fs module
//
// Example:
//   let config = parseFile("./config.ncl")
//   switch config {
//   | Ok(json) => Js.log(json)
//   | Error(err) => Js.log2("Error:", err)
//   }
@module("fs")
external readFileSync: (string, string) => string = "readFileSync"

let parseFile = (path: string): result<Js.Json.t, error> => {
  try {
    let source = readFileSync(path, "utf8")
    parseNickel(source, path)
  } catch {
  | _ => Error(InvalidInput("Failed to read file: " ++ path))
  }
}

// Helper: Validate Nickel file from filesystem
//
// Example:
//   let result = validateFile("./config.ncl")
//   switch result {
//   | Ok() => Js.log("Valid!")
//   | Error(err) => Js.log2("Invalid:", err)
//   }
let validateFile = (path: string): result<unit, error> => {
  try {
    let source = readFileSync(path, "utf8")
    validateNickel(source, path)
  } catch {
  | _ => Error(InvalidInput("Failed to read file: " ++ path))
  }
}

// Helper: Get config value by key path
// Example: getConfigValue(config, ["server", "port"])
let rec getConfigValue = (json: Js.Json.t, path: list<string>): option<Js.Json.t> => {
  switch path {
  | list{} => Some(json)
  | list{key, ...rest} =>
    switch Js.Json.decodeObject(json) {
    | Some(obj) =>
      switch Js.Dict.get(obj, key) {
      | Some(value) => getConfigValue(value, rest)
      | None => None
      }
    | None => None
    }
  }
}

// Helper: Convert error to string for display
let errorToString = (err: error): string => {
  switch err {
  | ParseError(msg) => "Parse Error: " ++ msg
  | ValidationError(msg) => "Validation Error: " ++ msg
  | InvalidInput(msg) => "Invalid Input: " ++ msg
  }
}

// Re-export result type for convenience
type parseResult = result<Js.Json.t, error>
type validateResult = result<unit, error>
