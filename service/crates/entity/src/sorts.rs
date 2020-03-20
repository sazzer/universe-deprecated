use std::str::FromStr;
use strum::VariantNames;
use unicase::UniCase;

/// Enumeration of possible sort orders for a search
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SortDirection {
  /// Sort in ascending order
  Ascending,
  /// Sort in descending order
  Descending,
  /// Sort in natural order for the field, which is either Ascending or Descending but depends on the field
  Natural,
}

/// Representation of a single field that can be sorted on
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SortField<T> {
  pub field: T,
  pub direction: SortDirection,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseSortsError {
  #[error("Unknown sort fields: {0:?}")]
  UnknownFields(Vec<String>),
}

/// Helper to return a string with the first character removed
/// Used when parsing a sort field that is prefixed with '+' or '-' to return the actual field name
fn remove_first(s: &str) -> &str {
  s.chars().next().map(|c| &s[c.len_utf8()..]).unwrap_or("")
}

impl<T> FromStr for SortField<T>
where
  T: VariantNames,
  T: FromStr,
{
  type Err = ParseSortsError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (direction, field_name) = match s.chars().next() {
      Some('+') => (SortDirection::Ascending, remove_first(s)),
      Some('-') => (SortDirection::Descending, remove_first(s)),
      _ => (SortDirection::Natural, s),
    };

    let unicase_field_name = UniCase::new(field_name);

    match T::VARIANTS.iter().find(|&v| UniCase::new(v) == unicase_field_name) {
      None => Err(ParseSortsError::UnknownFields(vec![s.to_owned()])),
      Some(matched_field) => {
        let field = T::from_str(matched_field)
          .map_err(|_| ParseSortsError::UnknownFields(vec![s.to_owned()]))?;
        Ok(SortField { field, direction })
      }
    }
  }
}

/// Parse a string into a list of sorts that we can use
pub fn parse_sorts<S, T>(input: S) -> Result<Vec<SortField<T>>, ParseSortsError>
where
  S: Into<String>,
  T: VariantNames,
  T: FromStr,
{
  let (fields, errors): (Vec<Result<SortField<T>, _>>, Vec<Result<_, ParseSortsError>>) = input.into()
    .split(",")
    .map(|sort| sort.trim())
    .filter(|sort| !sort.is_empty())
    .map(|sort| sort.parse::<SortField<T>>())
    .partition(Result::is_ok);

    if errors.is_empty() {
      Ok(fields.into_iter().map(Result::unwrap).collect())
    } else {
      let result = errors.into_iter().map(|e| e.map(|_| "This can't happen").unwrap_err())
        .fold(ParseSortsError::UnknownFields(vec![]),
        |acc, next| {
          match (acc, next) {
            (ParseSortsError::UnknownFields(old), ParseSortsError::UnknownFields(new)) => {
              let mut result = old.clone();
              result.extend(new);
              ParseSortsError::UnknownFields(result)
            }
          }
      });
      Err(result)
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;
  use spectral::prelude::*;
  use strum_macros::{EnumString, EnumVariantNames};

  #[derive(Debug, PartialEq, EnumVariantNames, EnumString)]
  #[allow(dead_code)]
  enum TestSorts {
    Name,
    Age,
    SerialNumber,
  }

  #[rstest(input, expected, 
    case("Name", Ok(SortField { field: TestSorts::Name, direction: SortDirection::Natural})),
    case("name", Ok(SortField { field: TestSorts::Name, direction: SortDirection::Natural})),
    case("NAME", Ok(SortField { field: TestSorts::Name, direction: SortDirection::Natural})),
    case("SerialNumber", Ok(SortField { field: TestSorts::SerialNumber, direction: SortDirection::Natural})),
    case("+name", Ok(SortField { field: TestSorts::Name, direction: SortDirection::Ascending})),
    case("-name", Ok(SortField { field: TestSorts::Name, direction: SortDirection::Descending})),
    case("unknown", Err(ParseSortsError::UnknownFields(vec!["unknown".to_owned()]))),
    case("+unknown", Err(ParseSortsError::UnknownFields(vec!["+unknown".to_owned()]))),
    case("-unknown", Err(ParseSortsError::UnknownFields(vec!["-unknown".to_owned()]))),
  )]
  fn test_parse_sort(input: &str, expected: Result<SortField<TestSorts>, ParseSortsError>) {
    let result: Result<SortField<TestSorts>, ParseSortsError> = input.parse();
    assert_that(&result).is_equal_to(expected);
  }

  #[rstest(input, expected, 
    case("", Ok(vec![])),
    case("Name", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Natural}
    ])),
    case("name", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Natural}
    ])),
    case("NAME", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Natural}
    ])),
    case("+name", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Ascending}
    ])),
    case("-name", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Descending}
    ])),
    case("  name  ", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Natural}
    ])),
    case(",,,", Ok(vec![])),
    case("name,age", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Natural},
      SortField { field: TestSorts::Age, direction: SortDirection::Natural},
    ])),
    case("+name,-age", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Ascending},
      SortField { field: TestSorts::Age, direction: SortDirection::Descending},
    ])),
    case("+name,age", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Ascending},
      SortField { field: TestSorts::Age, direction: SortDirection::Natural},
    ])),
    case("name, age", Ok(vec![
      SortField { field: TestSorts::Name, direction: SortDirection::Natural},
      SortField { field: TestSorts::Age, direction: SortDirection::Natural},
    ])),
    case("unknown", Err(ParseSortsError::UnknownFields(vec!["unknown".to_owned()]))),
    case("name,unknown", Err(ParseSortsError::UnknownFields(vec!["unknown".to_owned()]))),
    case("+name,-unknown", Err(ParseSortsError::UnknownFields(vec!["-unknown".to_owned()]))),
    case("unknown1,unknown2", Err(ParseSortsError::UnknownFields(vec!["unknown1".to_owned(), "unknown2".to_owned()]))),
  )]
  fn test_parse_sorts(input: &str, expected: Result<Vec<SortField<TestSorts>>, ParseSortsError>) {
    let result: Result<Vec<SortField<TestSorts>>, ParseSortsError> = parse_sorts(input);
    assert_that(&result).is_equal_to(expected);
  }

  #[test]
  fn test_parse_simple_sort() {
    let result: Result<Vec<SortField<TestSorts>>, ParseSortsError> = parse_sorts("Name");
    assert_that(&result).is_ok().is_equal_to(vec![SortField {
      field: TestSorts::Name,
      direction: SortDirection::Natural,
    }]);
  }
}
