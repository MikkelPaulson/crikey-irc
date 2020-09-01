use super::ParseError;
use std::result::Result;
use std::str::FromStr;

const QUERY_LIST: char = 'l';
const QUERY_USAGE_COUNT: char = 'm';
const QUERY_OPS: char = 'o';
const QUERY_UPTIME: char = 'u';

#[derive(PartialEq, Debug)]
pub enum StatsQuery {
    List,
    UsageCount,
    Ops,
    Uptime,
    Unknown(char),
}

impl FromStr for StatsQuery {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() == 1 {
            match raw.chars().nth(0) {
                Some(QUERY_LIST) => Ok(StatsQuery::List),
                Some(QUERY_USAGE_COUNT) => Ok(StatsQuery::UsageCount),
                Some(QUERY_OPS) => Ok(StatsQuery::Ops),
                Some(QUERY_UPTIME) => Ok(StatsQuery::Uptime),
                Some(c) if c.is_ascii_alphanumeric() => Ok(StatsQuery::Unknown(c)),
                _ => Err(ParseError::new("StatsQuery")),
            }
        } else {
            Err(ParseError::new("StatsQuery"))
        }
    }
}

impl From<StatsQuery> for String {
    fn from(stats_query: StatsQuery) -> String {
        let mut result = String::new();
        result.push(match stats_query {
            StatsQuery::List => QUERY_LIST,
            StatsQuery::UsageCount => QUERY_USAGE_COUNT,
            StatsQuery::Ops => QUERY_OPS,
            StatsQuery::Uptime => QUERY_UPTIME,
            StatsQuery::Unknown(c) => c,
        });
        result
    }
}

#[cfg(test)]
mod test_stats_query {
    use super::*;

    #[test]
    fn invalid() {
        assert!("".parse::<StatsQuery>().is_err());
        assert!("ab".parse::<StatsQuery>().is_err());
        assert!("🥔️".parse::<StatsQuery>().is_err());
        assert!("\0".parse::<StatsQuery>().is_err());
        assert!("-".parse::<StatsQuery>().is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(
            Ok(StatsQuery::List),
            "l".parse::<StatsQuery>()
        );
        assert_eq!(
            Ok(StatsQuery::UsageCount),
            "m".parse::<StatsQuery>()
        );
        assert_eq!(
            Ok(StatsQuery::Ops),
            "o".parse::<StatsQuery>()
        );
        assert_eq!(
            Ok(StatsQuery::Uptime),
            "u".parse::<StatsQuery>()
        );
        assert_eq!(
            Ok(StatsQuery::Unknown('a')),
            "a".parse::<StatsQuery>()
        );
        assert_eq!(
            Ok(StatsQuery::Unknown('0')),
            "0".parse::<StatsQuery>()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            "l".to_string(),
            String::from(StatsQuery::List)
        );
        assert_eq!(
            "m".to_string(),
            String::from(StatsQuery::UsageCount)
        );
        assert_eq!(
            "o".to_string(),
            String::from(StatsQuery::Ops)
        );
        assert_eq!(
            "u".to_string(),
            String::from(StatsQuery::Uptime)
        );
        assert_eq!(
            "a".to_string(),
            String::from(StatsQuery::Unknown('a'))
        );
        assert_eq!(
            "0".to_string(),
            String::from(StatsQuery::Unknown('0'))
        );
    }
}
