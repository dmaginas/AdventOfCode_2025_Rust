use std::fmt;

/// Represents an error while parsing a rotation instruction.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    EmptyLine,
    MissingDirection,
    InvalidDirection(char),
    InvalidDistance(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "empty line"),
            ParseError::MissingDirection => write!(f, "missing direction"),
            ParseError::InvalidDirection(dir) => write!(f, "invalid direction `{dir}`"),
            ParseError::InvalidDistance(err) => write!(f, "invalid distance: {err}"),
        }
    }
}

/// Counts how many times the dial points at 0 after applying all rotations in the
/// provided input.
///
/// The input is expected to contain one rotation per line in the format `L68` or `R14`.
/// Empty lines are ignored.
pub fn solve(input: &str) -> Result<usize, String> {
    let mut position: i32 = 50;
    let mut zero_hits: usize = 0;

    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let delta = parse_rotation(line).map_err(|err| format!("Line {}: {err}", idx + 1))?;

        position = (position + delta).rem_euclid(100);
        if position == 0 {
            zero_hits += 1;
        }
    }

    Ok(zero_hits)
}

fn parse_rotation(line: &str) -> Result<i32, ParseError> {
    let mut chars = line.chars();
    let direction = chars.next().ok_or(ParseError::MissingDirection)?;
    let distance_str = chars.as_str();

    if distance_str.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    let distance: i32 = distance_str
        .parse::<i32>()
        .map_err(|e| ParseError::InvalidDistance(e.to_string()))?;

    match direction {
        'L' => Ok(-distance),
        'R' => Ok(distance),
        other => Err(ParseError::InvalidDirection(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";

    #[test]
    fn sample_is_three() {
        let result = solve(SAMPLE).expect("should parse");
        assert_eq!(result, 3);
    }

    #[test]
    fn wraps_correctly() {
        let input = "L60\nR150\n"; // 50 -> 90 -> 40
        let result = solve(input).expect("should parse");
        assert_eq!(result, 0);
    }

    #[test]
    fn reports_invalid_direction() {
        let err = solve("X10\n").unwrap_err();
        assert!(err.contains("invalid direction"));
    }
}
