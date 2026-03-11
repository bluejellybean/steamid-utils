use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SteamIdKind {
    Steam64,
    Steam32,
    Steam3,
}

pub fn parse_incoming_format(steam_id: &str) -> Result<SteamIdKind, SteamIdError> {
    if steam_id.len() == 17 && steam_id.starts_with("7656119") {
        return Ok(SteamIdKind::Steam64);
    }

    if steam_id.to_ascii_uppercase().starts_with("STEAM_") {
        return Ok(SteamIdKind::Steam32);
    }

    if steam_id.starts_with("[U:") && steam_id.ends_with(']') {
        return Ok(SteamIdKind::Steam3);
    }

    return Err(SteamIdError::InvalidFormat);
}

pub fn to_steam32(steam_id: &str) -> Result<String, SteamIdError> {
    let incoming_format = parse_incoming_format(steam_id)?; // ← unwrap here with ?

    match incoming_format {
        SteamIdKind::Steam64 => {
            let steamid64: u64 = steam_id.parse()?;

            const UNIVERSE_BASE: u64 = 76561197960265728;
            if steamid64 < UNIVERSE_BASE {
                return Err(SteamIdError::InvalidFormat);
            }

            let account_id = steamid64 - UNIVERSE_BASE;
            let middle_digit = account_id % 2;
            let auth_server = account_id / 2;

            Ok(format!("STEAM_0:{}:{}", middle_digit, auth_server))
        }

        SteamIdKind::Steam32 => Ok(steam_id.to_string()),

        SteamIdKind::Steam3 => {
            let input = steam_id.trim();

            let cleaned = if input.starts_with('[') && input.ends_with(']') {
                &input[1..input.len() - 1]
            } else {
                input
            };

            let parts: Vec<&str> = cleaned.split(':').collect();
            if parts.len() != 3 {
                return Err(SteamIdError::InvalidFormat);
            }

            if parts[0] != "U" {
                return Err(SteamIdError::InvalidPrefix);
            }

            if parts[1] != "1" {
                return Err(SteamIdError::InvalidFormat);
            }

            let account_id_str = parts[2];
            let account_id: u32 = account_id_str.parse().map_err(SteamIdError::ParseError)?;
            let y = account_id % 2;
            let z = account_id / 2;

            Ok(format!("STEAM_0:{}:{}", y, z))
        }
    }
}

pub fn to_steam64(steam_id: &str) -> Result<String, SteamIdError> {
    let incoming_format = parse_incoming_format(steam_id)?;
    match incoming_format {
        SteamIdKind::Steam64 => Ok(steam_id.to_string()),
        SteamIdKind::Steam32 => {
            const STEAMID64_BASE: u64 = 76561197960265728;
            let parts: Vec<&str> = steam_id.split(':').collect();
            let y: u32 = parts[1].parse().map_err(|_| SteamIdError::InvalidY)?;

            let z: u64 = parts[2].parse().map_err(|_| SteamIdError::InvalidZ)?;

            if y > 1 {
                return Err(SteamIdError::InvalidY);
            }
            let account_id = z * 2 + u64::from(y);

            let steam64 = STEAMID64_BASE + account_id;

            Ok(steam64.to_string())
        }
        SteamIdKind::Steam3 => {
            let input = steam_id.trim();
            let cleaned = if input.starts_with('[') && input.ends_with(']') {
                &input[1..input.len() - 1]
            } else {
                input
            };

            let parts: Vec<&str> = cleaned.split(':').collect();
            if parts.len() != 3 {
                return Err(SteamIdError::InvalidFormat);
            }

            let account_id: u64 = parts[2].parse().map_err(SteamIdError::ParseError)?;

            if account_id == 0 {
                return Err(SteamIdError::InvalidFormat);
            }

            const STEAMID64_BASE: u64 = 76561197960265728;
            let steam64 = STEAMID64_BASE + account_id;

            Ok(steam64.to_string())
        }
    }
}

pub fn to_steam3(steam_id: &str) -> Result<String, SteamIdError> {
    let incoming_format = parse_incoming_format(steam_id)?;
    match incoming_format {
        SteamIdKind::Steam64 => {
            const STEAMID64_BASE: u64 = 76561197960265728;
            let id64: u64 = steam_id.parse().map_err(SteamIdError::ParseError)?;
            if id64 < STEAMID64_BASE {
                return Err(SteamIdError::InvalidFormat);
            }
            let account_id = id64 - STEAMID64_BASE;
            Ok(format!("[U:1:{}]", account_id))
        }
        SteamIdKind::Steam32 => {
            let parts: Vec<&str> = steam_id.split(':').collect();
            if parts.len() != 3 {
                return Err(SteamIdError::InvalidFormat);
            }
            let y: u32 = parts[1].parse().map_err(SteamIdError::ParseError)?;
            let z: u32 = parts[2].parse().map_err(SteamIdError::ParseError)?;

            if y > 1 {
                return Err(SteamIdError::InvalidFormat);
            }
            let account_id = z * 2 + y;
            Ok(format!("[U:1:{}]", account_id))
        }
        SteamIdKind::Steam3 => Ok(steam_id.to_string()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SteamIdError {
    InvalidFormat,
    InvalidPrefix,
    InvalidY,
    InvalidZ,
    ParseError(ParseIntError),
}

impl std::fmt::Display for SteamIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat => {
                write!(f, "SteamID32 must have exactly 3 parts separated by ':'")
            }
            Self::InvalidPrefix => write!(f, "SteamID32 must start with 'STEAM_0' or 'STEAM_1'"),
            Self::InvalidY => write!(f, "The second part (Y) must be 0 or 1"),
            Self::InvalidZ => write!(f, "The third part (Z) must be a valid non-negative integer"),
            Self::ParseError(e) => write!(f, "Number parsing failed: {}", e),
        }
    }
}
impl std::error::Error for SteamIdError {}

impl From<ParseIntError> for SteamIdError {
    fn from(err: ParseIntError) -> Self {
        SteamIdError::ParseError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_to_steam32() {
        assert_eq!(
            to_steam32("STEAM_0:1:11110394"),
            Ok("STEAM_0:1:11110394".to_string())
        );
        assert_eq!(
            to_steam32("76561197982486517"),
            Ok("STEAM_0:1:11110394".to_string())
        );
        assert_eq!(
            to_steam32("[U:1:22220789]"),
            Ok("STEAM_0:1:11110394".to_string())
        );
    }
    #[test]
    fn test_to_steam64() {
        assert_eq!(
            to_steam64("STEAM_0:1:11110394"),
            Ok("76561197982486517".to_string())
        );
        assert_eq!(
            to_steam64("76561197982486517"),
            Ok("76561197982486517".to_string())
        );
        assert_eq!(
            to_steam64("[U:1:22220789]"),
            Ok("76561197982486517".to_string())
        );
    }

    #[test]
    fn test_to_steam3() {
        assert_eq!(
            to_steam3("[U:1:22220789]"),
            Ok("[U:1:22220789]".to_string())
        );
        assert_eq!(
            to_steam3("76561197982486517"),
            Ok("[U:1:22220789]".to_string())
        );
        assert_eq!(
            to_steam3("STEAM_0:1:11110394"),
            Ok("[U:1:22220789]".to_string())
        );
    }
    #[test]
    fn test_parse_incoming_format() {
        assert_eq!(
            parse_incoming_format("76561197982486517"),
            Ok(SteamIdKind::Steam64)
        );
        assert_eq!(
            parse_incoming_format("[U:1:22220789]"),
            Ok(SteamIdKind::Steam3)
        );
        assert_eq!(
            parse_incoming_format("STEAM_0:1:11110394"),
            Ok(SteamIdKind::Steam32)
        );

        assert_eq!(
            parse_incoming_format("7656119798248"),
            Err(SteamIdError::InvalidFormat)
        );
        assert_eq!(
            parse_incoming_format("[STEAM_0:1:11110394]"),
            Err(SteamIdError::InvalidFormat)
        );
        assert_eq!(
            parse_incoming_format("U:1:22220789"),
            Err(SteamIdError::InvalidFormat)
        );
    }
}
