# steamid-utils

```steamid-utils``` converts any valid string versrion of a steamID to any other valid steamID.


---

Usage guide parse_incoming_format():

```
use steamid_parser::{parse_incoming_format, SteamIdKind, SteamIdError};

fn main() {
    let input = "76561197960265728";

    match parse_incoming_format(input) {
        Ok(kind) => {
            println!("Valid Steam ID detected: {:?}", kind);
            // Example output: Valid Steam ID detected: Steam64
        }
        Err(e) => {
            eprintln!("Invalid Steam ID: {}", e);
        }
    }

    // You can also do short aliases if you want
    use steamid_parser::SteamIdKind as IdType;

    let id_type = parse_incoming_format("STEAM_1:0:12345678").unwrap();
    assert_eq!(id_type, IdType::Steam32);
}
```


Usage steam64/32/3():

```
use steamid_parser::{to_steam32, SteamIdError};

fn convert_and_show(steam_id: &str) {
    println!("Input:  {}", steam_id);
    
    match to_steam32(steam_id) {
        Ok(converted) => {
            println!("Steam32: {}", converted);
            println!("Success ✓");
        }
        Err(SteamIdError::InvalidFormat) => {
            eprintln!("→ Invalid format (not a recognized Steam ID)");
        }
        Err(SteamIdError::InvalidPrefix) => {
            eprintln!("→ Not a valid Steam3 ID (must start with [U:1:...)");
        }
        Err(SteamIdError::ParseError(e)) => {
            eprintln!("→ Number parsing failed: {}", e);
        }
        Err(other) => {
            eprintln!("→ Unexpected error: {}", other);
        }
    }
    println!("─".repeat(40));
}

fn main() {
    let examples = [
        "76561197960287930",
        "STEAM_0:0:12345",
        "[U:1:246824]",
        "76561190000000000",   // too small / invalid
        "[U:1:abc]",           // not a number
        "invalid",
    ];

    for id in examples {
        convert_and_show(id);
    }
}
```

# Contribute

Pull requests are welcomed and encouraged!

---

If you have any questions, suggestions, or bugs reports please feel free to open an issue.

