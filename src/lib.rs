use pulldown_cmark;
use pulldown_cmark::{Event, Options, Parser};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
#[pyo3(name = "find_html_positions")]
/// Find positions of html nodes in a markdown file
///
/// Returns a list of tuples of start and end positions
///
/// Raises ValueError when passed carriage returns. You must strip carriage
/// returns before calling this function.
fn find_html_positions_py(markdown: &str) -> PyResult<Vec<(usize, usize)>> {
    find_html_positions(markdown).map_err(|e| PyValueError::new_err(e))
}

/// Module for finding HTML from markdown
#[pymodule]
fn markdown_html_finder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_html_positions_py))?;

    Ok(())
}

fn join_adjacent_spans(spans: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_spans: Vec<(usize, usize)> = Vec::new();
    for (span_start, span_end) in spans.iter() {
        if let Some(prev_span) = new_spans.last_mut() {
            let (prev_start, prev_end) = *prev_span;
            if prev_end >= *span_start {
                *prev_span = (prev_start, *span_end);
                continue;
            }
        }
        new_spans.push((*span_start, *span_end));
    }
    new_spans
}

#[test]
fn test_join_adjacent_spans() {
    let source = vec![(1, 3), (3, 6), (10, 12), (15, 17), (16, 18)];
    let expected = vec![(1, 6), (10, 12), (15, 18)];
    assert_eq!(join_adjacent_spans(source), (expected));
}

fn find_html_positions(markdown: &str) -> Result<Vec<(usize, usize)>, String> {
    if markdown.chars().any(|x| x == '\r') {
        return Err(
            "carriage returns are unsupported, please strip them from your input.".to_string(),
        );
    }

    let results = Parser::new_ext(markdown, Options::empty())
        .into_offset_iter()
        .filter(|(event, _range)| match event {
            Event::Html(..) | Event::InlineHtml(..) | Event::SoftBreak | Event::HardBreak => true,
            _ => false,
        })
        .map(|(_event, range)| (range.start, range.end))
        .collect();
    Ok(join_adjacent_spans(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_html_positions_simple() {
        let source = "1234567<!-- comment text --> 123";
        let expected = vec![(7, 28)];
        assert_eq!(find_html_positions(source), Ok(expected));
    }
    #[test]
    fn find_html_positions_complex() {
        let source = r##"testing
<p align=center><img src="https://github.com/chdsbd/kodiak/raw/master/assets/logo.png" alt="" width="200" height="200"><!-- comment-a --></p><!-- comment-b -->test
hello"##;
        let expected = vec![(8, 177)];
        assert_eq!(find_html_positions(source), Ok(expected));
    }
    #[test]
    fn find_html_positions_complex_two() {
        let source = "hello <span>  <p>  <!-- testing --> hello</p></span>world";
        let expected = vec![(6, 12), (14, 17), (19, 35), (41, 52)];
        assert_eq!(find_html_positions(source), Ok(expected));
    }
    #[test]
    fn find_html_positions_complex_three() {
        let source = r##"
Non dolor velit vel quia mollitia. Placeat cumque a deleniti possimus.

Totam dolor [exercitationem laborum](https://numquam.com)

<!--
- Voluptatem voluptas officiis
- Voluptates nulla tempora
- Officia distinctio ut ab
  + Est ut voluptatum consequuntur recusandae aspernatur
  + Quidem debitis atque dolorum est enim
-->
"##;
        let expected = vec![(132, 325)];
        assert_eq!(find_html_positions(source), Ok(expected));
    }
    #[test]
    fn find_html_positions_carriage_return() {
        let source = "testing 123 \r\n\r<-- some comment\r\n with carriage returns-->";
        assert!(find_html_positions(source).is_err());
    }
    #[test]
    fn fenced_code_block() {
        let source = r##"hello
```
<-- foo -->
```
"##;
        let expected: Vec<(usize, usize)> = vec![];
        assert_eq!(find_html_positions(source), Ok(expected));
    }
}
