use pulldown_cmark;
use pulldown_cmark::{Event, Options, Parser};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Module for finding HTML from markdown
#[pymodule]
fn markdown_html_finder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_html_positions_py))?;

    #[pyfn(m, "find_html_positions")]
    /// Find positions of html nodes in a markdown file
    ///
    /// Returns a list of tuples of start and end positions
    fn find_html_positions_py(markdown: &'static str) -> PyResult<Vec<(usize, usize)>> {
        match find_html_positions(markdown) {
            Ok(result) => Ok(result),
            Err(err_message) => Err(exceptions::ValueError::py_err(err_message)),
        }
    }

    Ok(())
}

fn find_html_positions(markdown: &str) -> Result<Vec<(usize, usize)>, &str> {
    if markdown.chars().find(|&x| x == '\r').is_some() {
        return Err("carriage returns are unsupported, please strip them from your input.");
    }

    let results = Parser::new_ext(markdown, Options::empty())
        .into_offset_iter()
        .filter(|(event, range)| match event {
            Event::Html(..) | Event::InlineHtml(..) | Event::SoftBreak | Event::HardBreak => true,
            _ => false,
        })
        .map(|(_event, range)| (range.start, range.end))
        .collect();
    Ok(results)
    // TODO(chdsbd): combine adjacent spans
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
        let expected = vec![(8, 172), (172, 177)];
        assert_eq!(find_html_positions(source), Ok(expected));
    }
    #[test]
    fn find_html_positions_complex_two() {
        let source = "hello <span>  <p>  <!-- testing --> hello</p></span>world";
        let expected = vec![(6, 52)];
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
