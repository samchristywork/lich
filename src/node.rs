use crate::environment::Environment;
use chrono::TimeZone;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node {
    Symbol(String),
    Number(i64),
    Text(String),
    Bool(bool),
    List(Vec<Node>),
    Time(i64, i32), // Seconds since epoch and timezone offset in seconds
    Function(fn(&[Node], &mut Environment) -> Result<Node, String>),
    Regex(String), // TODO: It would be more efficient to store a compiled regex
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Time(t, z) => {
                let local_time = chrono::Utc
                    .timestamp_opt(*t, 0)
                    .single()
                    .expect("Invalid timestamp");
                let local_time_str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
                let offset_hours = z / 3600;
                let offset_minutes = (z % 3600) / 60;
                format!("{local_time_str} UTC{offset_hours:+03}:{offset_minutes:02}")
            }
            Self::Text(s) | Self::Symbol(s) => s.clone(),
            Self::Function(_) => "function".to_string(),
            Self::List(nodes) => {
                let mut result = String::new();
                result.push('(');
                result.push_str(
                    &nodes
                        .iter()
                        .map(Self::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                result.push(')');
                result
            }
            Self::Regex(r) => format!("regex({r})"),
        };

        if res.is_empty() {
            write!(f, "nil")
        } else {
            write!(f, "{res}")
        }
    }
}
