use crate::node::Node;
use crate::environment::Environment;

//- (test "time" (time->string (time 2025 1 1 12 0 0 -5)) "2025-01-01 12:00:00 UTC-05:00")
//- (test "time" (time->number (time 1970 1 1 0 0 0 0)) 0)
pub fn fn_time(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.len() != 7 {
        return Err("Invalid number of arguments for time".to_string());
    }

    let mut args = Vec::new();
    for arg in arguments {
        if let Node::Number(n) = arg {
            args.push(*n);
        } else {
            return Err("Invalid argument type for time. Expected Number.".to_string());
        }
    }

    let year = args[0] as i32;
    let month = args[1] as u32;
    let day = args[2] as u32;
    let hour = args[3] as u32;
    let minute = args[4] as u32;
    let second = args[5] as u32;
    let offset = args[6];

    let dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|d| d.and_hms_opt(hour, minute, second))
        .ok_or("Invalid date/time values".to_string())?;

    let dt_seconds = dt.and_utc().timestamp();

    let offset_seconds = (offset * 3600)
        .try_into()
        .map_err(|_| "Offset value too large".to_string())?;

    Ok(Node::Time(dt_seconds, offset_seconds))
}

pub fn fn_now(arguments: &[Node], _: &mut Environment) -> Result<Node, String> {
    if arguments.is_empty() {
        let now = chrono::Utc::now();
        let now_seconds = now.timestamp();
        let tz = chrono::Local::now()
            .offset()
            .local_minus_utc();
        return Ok(Node::Time(now_seconds, tz));
    }

    Err("Invalid arguments for now".to_string())
}

//- (test "add-days" (add-days 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 2 12 0 0 -5))
//- (test "add-days" (add-days -1 (time 2025 1 1 12 0 0 -5)) (time 2024 12 31 12 0 0 -5))
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

//- (test "add-hours" (add-hours 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 13 0 0 -5))
//- (test "add-hours" (add-hours -1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 11 0 0 -5))
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

//- (test "add-minutes" (add-minutes 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 12 1 0 -5))
//- (test "add-minutes" (add-minutes -1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 11 59 0 -5))
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

//- (test "add-seconds" (add-seconds 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 12 0 1 -5))
//- (test "add-seconds" (add-seconds -1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 11 59 59 -5))
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
