use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ConnectionString {
    pub name: String,
    pub uri: String,
}

impl FromStr for ConnectionString {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(2, [',', '=']).collect();
        if parts.len() != 2 {
            return Err("Expected format <name,uri>".into());
        }
        Ok(ConnectionString {
            name: parts[0].to_string(),
            uri: parts[1].to_string(),
        })
    }
}
