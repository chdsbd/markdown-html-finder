# markdown-html-finder
An experiment with writing Python modules in Rust using [pyo3](https://github.com/PyO3/pyo3) and [pyo3-pack](https://github.com/PyO3/pyo3-pack).

## install

```bash
# poetry
poetry add markdown-html-finder

# pip
pip install markdown-html-finder
```

## usage

```python
from markdown_html_finder import find_html_positions

DOCUMENT = """\
# example markdown document

Amet nobis et numquam qui. Animi perferendis quia qui ut aut expedita. Ut eveniet quia quaerat.
<!-- hello world -->
Quisquam et et velit soluta quia.
"""

# NOTE: find_html_positions raises a ValueError if passed carriage returns `\r`
stripped_document = DOCUMENT.replace('\r', '')
html_positions = find_html_positions(stripped_document)
assert html_positions == [(125, 145)]
```

## dev
```bash
# install build dependencies
poetry install

# build for python development
poetry run pyo3-pack development

# build for release
poetry run pyo3-pack build

# publish
poetry run pyo3-pack publish
```

### markdown-html-finder-builder build
```bash
VERSION='0.2.0'
docker build -f build.Dockerfile . --tag cdignam/markdown-html-finder-builder:$VERSION
```

### production build
Update the version in Cargo.toml and run the following commands to build and upload linux wheels.
```bash
VERSION='0.2.0'

# build manylinux wheels 
docker run --rm -v $(pwd):/io cdignam/markdown-html-finder-builder:$VERSION   build --release

# build macos wheel (only builds for installed Python version)
pyo3-pack build --release

# upload built wheels to pypi
twine upload target/wheels/* --skip-existing
```
