use pulldown_cmark;
use pulldown_cmark::{Event, Options, Parser};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
/// Find positions of html nodes in a markdown file
/// 
/// Returns a list of tuples of start and end positions
fn find_html_positions_py(markdown: &str) -> PyResult<Vec<(usize,usize)>> {
     Ok(find_html_positions(markdown))
}

/// Module for finding HTML from markdown
#[pymodule]
fn markdown_html_finder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_html_positions_py))?;

    Ok(())
}



fn find_html_positions(markdown: &str) -> Vec<(usize,usize)> {
    Parser::new_ext(markdown, Options::empty()).into_offset_iter()
        .filter(|(event, _range)| match event {
            Event::Html(..) | Event::InlineHtml(..) => true,
            _ => false,
        })
        .map(|(_event, range)|  {
            (range.start,range.end)
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_html_positions_simple() {
        let source = "1234567<!-- comment text --> 123";
        let expected = vec![(
            7,
            28
        )];
        assert_eq!(find_html_positions(source), expected);
    }
    #[test]
    fn find_html_positions_complex() {
        let source = r##"testing
<p align=center><img src="https://github.com/chdsbd/kodiak/raw/master/assets/logo.png" alt="" width="200" height="200"><!-- comment-a --></p><!-- comment-b -->test
hello"##;
        let expected = vec![(8, 172),(
            172,
            177
        )];
        assert_eq!(find_html_positions(source), expected);
    }
    #[test]
    fn fenced_code_block() {
        let source = r##"hello
```
<-- foo -->
```
"##;    
        let expected: Vec<(usize, usize)> = vec![];
        assert_eq!(find_html_positions(source), expected);
    }
}
