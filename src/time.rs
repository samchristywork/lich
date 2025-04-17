use crate::node::Node;
use crate::environment::Environment;
use chrono::Offset;

pub fn fn_time(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 1 {
        if let Node::Text(s) = &arguments[0] {
            if let Ok(time) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                let seconds = time.timestamp();
                let local_time_zone_minutes_offset = chrono::Local::now()
                    .offset()
                    .local_minus_utc();
                return Ok(Node::Time(seconds, local_time_zone_minutes_offset));
            }
        }
    }

    Err("Invalid arguments for time".to_string())
}

pub fn fn_now(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.is_empty() {
        let now = chrono::Utc::now();
        let now_seconds = now.timestamp();
        let local_time_zone_minutes_offset = chrono::Local::now()
            .offset()
            .local_minus_utc();
        return Ok(Node::Time(now_seconds, local_time_zone_minutes_offset));
    }

    Err("Invalid arguments for now".to_string())
}

pub fn fn_add_days(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Time(seconds, offset) = &arguments[1] {
            if let Node::Number(days) = &arguments[0] {
                let new_seconds = *seconds + days * 86400;
                return Ok(Node::Time(new_seconds, *offset));
            }
        }
    }

    Err("Invalid arguments for add-days".to_string())
}

pub fn fn_add_hours(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Time(seconds, offset) = &arguments[1] {
            if let Node::Number(hours) = &arguments[0] {
                let new_seconds = *seconds + hours * 3600;
                return Ok(Node::Time(new_seconds, *offset));
            }
        }
    }

    Err("Invalid arguments for add-hours".to_string())
}

pub fn fn_add_minutes(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Time(seconds, offset) = &arguments[1] {
            if let Node::Number(minutes) = &arguments[0] {
                let new_seconds = *seconds + minutes * 60;
                return Ok(Node::Time(new_seconds, *offset));
            }
        }
    }

    Err("Invalid arguments for add-minutes".to_string())
}

pub fn fn_add_seconds(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() == 2 {
        if let Node::Time(seconds, offset) = &arguments[1] {
            if let Node::Number(seconds_to_add) = &arguments[0] {
                let new_seconds = *seconds + seconds_to_add;
                return Ok(Node::Time(new_seconds, *offset));
            }
        }
    }

    Err("Invalid arguments for add-seconds".to_string())
}
