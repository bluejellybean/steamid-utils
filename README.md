# steamid-utils

```steamid-utils``` converts any valid string versrion of a steamID to any other valid steamID.


---



```
use steamid_parser::{parse_incoming_format, SteamIdKind, SteamIdError};
//               ^^^^^^^^^^^^^^  this is your crate name (hyphens become underscores)

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


# Contribute

Pull requests are welcomed and encouraged!

---

If you have any questions, suggestions, or bugs reports please feel free to open an issue.

