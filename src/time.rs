use crate::invalid_arguments;
use crate::node::Node;

//- (test "time" (time->string (time 2025 1 1 12 0 0 -5)) "2025-01-01 12:00:00 UTC-05:00")
//- (test "time" (time->number (time 1970 1 1 0 0 0 0)) 0)
pub fn fn_time(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(year), Node::Number(month), Node::Number(day)] => {
            let year = i32::try_from(*year).map_err(|_| "Invalid year")?;
            let month = u32::try_from(*month).map_err(|_| "Invalid month")?;
            let day = u32::try_from(*day).map_err(|_| "Invalid day")?;

            let Some(dt) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return Err("Invalid date".to_string());
            };
            let Some(dt_seconds) = dt.and_hms_opt(0, 0, 0) else {
                return Err("Invalid time".to_string());
            };
            Ok(Node::Time(dt_seconds.and_utc().timestamp(), 0))
        }
        [
            Node::Number(year),
            Node::Number(month),
            Node::Number(day),
            Node::Number(hour),
            Node::Number(minute),
            Node::Number(second),
        ] => {
            let year = i32::try_from(*year).map_err(|_| "Invalid year")?;
            let month = u32::try_from(*month).map_err(|_| "Invalid month")?;
            let day = u32::try_from(*day).map_err(|_| "Invalid day")?;
            let hour = u32::try_from(*hour).map_err(|_| "Invalid hour")?;
            let minute = u32::try_from(*minute).map_err(|_| "Invalid minute")?;
            let second = u32::try_from(*second).map_err(|_| "Invalid second")?;

            let Some(dt) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return Err("Invalid date".to_string());
            };
            let Some(dt_seconds) = dt.and_hms_opt(hour, minute, second) else {
                return Err("Invalid time".to_string());
            };
            Ok(Node::Time(dt_seconds.and_utc().timestamp(), 0))
        }
        [
            Node::Number(year),
            Node::Number(month),
            Node::Number(day),
            Node::Number(hour),
            Node::Number(minute),
            Node::Number(second),
            Node::Number(offset),
        ] => {
            let year = i32::try_from(*year).map_err(|_| "Invalid year")?;
            let month = u32::try_from(*month).map_err(|_| "Invalid month")?;
            let day = u32::try_from(*day).map_err(|_| "Invalid day")?;
            let hour = u32::try_from(*hour).map_err(|_| "Invalid hour")?;
            let minute = u32::try_from(*minute).map_err(|_| "Invalid minute")?;
            let second = u32::try_from(*second).map_err(|_| "Invalid second")?;
            let offset = i32::try_from(*offset).map_err(|_| "Invalid offset")?;

            let Some(dt) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return Err("Invalid date".to_string());
            };
            let Some(dt_seconds) = dt.and_hms_opt(hour, minute, second) else {
                return Err("Invalid time".to_string());
            };
            Ok(Node::Time(dt_seconds.and_utc().timestamp(), offset * 3600))
        }
        _ => invalid_arguments!(
            "time",
            arguments,
            [
                "[Number(year), Number(month), Number(day), Number(hour), Number(minute), Number(second), Number(offset)]",
                "[Number(year), Number(month), Number(day), Number(hour), Number(minute), Number(second)]",
                "[Number(year), Number(month), Number(day)]",
            ]
        ),
    }
}

pub fn fn_now(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [] => {
            let now = chrono::Local::now();
            let offset = now.offset().local_minus_utc();
            let seconds = now.timestamp() + i64::from(offset);
            Ok(Node::Time(seconds, offset))
        }
        _ => invalid_arguments!("now", arguments, ["[]"]),
    }
}

//- (test "add-days" (add-days 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 2 12 0 0 -5))
//- (test "add-days" (add-days -1 (time 2025 1 1 12 0 0 -5)) (time 2024 12 31 12 0 0 -5))
pub fn fn_add_days(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(days), Node::Time(seconds, offset)] => {
            let new_seconds = *seconds + days * 86400;
            Ok(Node::Time(new_seconds, *offset))
        }
        _ => invalid_arguments!(
            "add-days",
            arguments,
            ["[Number(days), Time(seconds, offset)]"]
        ),
    }
}

//- (test "add-hours" (add-hours 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 13 0 0 -5))
//- (test "add-hours" (add-hours -1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 11 0 0 -5))
pub fn fn_add_hours(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(hours), Node::Time(seconds, offset)] => {
            let new_seconds = *seconds + hours * 3600;
            Ok(Node::Time(new_seconds, *offset))
        }
        _ => invalid_arguments!(
            "add-hours",
            arguments,
            ["[Number(hours), Time(seconds, offset)]"]
        ),
    }
}

//- (test "add-minutes" (add-minutes 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 12 1 0 -5))
//- (test "add-minutes" (add-minutes -1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 11 59 0 -5))
pub fn fn_add_minutes(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(minutes), Node::Time(seconds, offset)] => {
            let new_seconds = *seconds + minutes * 60;
            Ok(Node::Time(new_seconds, *offset))
        }
        _ => invalid_arguments!(
            "add-minutes",
            arguments,
            ["[Number(minutes), Time(seconds, offset)]"]
        ),
    }
}

//- (test "add-seconds" (add-seconds 1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 12 0 1 -5))
//- (test "add-seconds" (add-seconds -1 (time 2025 1 1 12 0 0 -5)) (time 2025 1 1 11 59 59 -5))
pub fn fn_add_seconds(arguments: &[Node]) -> Result<Node, String> {
    match arguments {
        [Node::Number(seconds), Node::Time(current_seconds, offset)] => {
            let new_seconds = *current_seconds + seconds;
            Ok(Node::Time(new_seconds, *offset))
        }
        _ => invalid_arguments!(
            "add-seconds",
            arguments,
            ["[Number(seconds), Time(current_seconds, offset)]"]
        ),
    }
}
