# markdown-html-finder [![PyPI](https://img.shields.io/pypi/v/markdown-html-finder.svg)](https://pypi.org/project/markdown-html-finder/)

A Python library to locate HTML spans in markdown text. This library is written in Rust with bindings for Python.

## why?
For a [separate project](https://github.com/chdsbd/kodiak) I needed to locate HTML comments in markdown documents. Sadly the markdown parsers I found for Python didn't provide span information for nodes.

While it wouldn't be too hard to add some features to existing Python markdown parsers, I thought it would be interesting to see how Rust can be used from Python. The excellent [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) crate provides span information for HTML elements, so that's what we use here.

[pyo3](https://github.com/PyO3/pyo3) and [pyo3-pack](https://github.com/PyO3/pyo3-pack) do the hard work of providing bindings to Python and building wheels to distribute on PyPi.

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

### markdown-html-finder-builder
This container extends the [quay.io/pypa/manylinux2010_x86_64](https://quay.io/pypa/manylinux2010_x86_64) docker image and is based on the [konstin2/pyo3-pack](https://hub.docker.com/r/konstin2/pyo3-pack) image, with Python2 support removed.

This image is built and uploaded manually to Docker Hub when necessary.

```bash
# build and publish a new version
VERSION='0.2.0'
docker build -f build.Dockerfile . --tag cdignam/markdown-html-finder-builder:$VERSION
docker push cdignam/markdown-html-finder-builder:$VERSION
```

### production build
Update the version in Cargo.toml and run the following commands to build and upload linux wheels.

```bash
VERSION='0.2.0'

# build manylinux wheels 
docker run --rm -v $(pwd):/io cdignam/markdown-html-finder-builder:$VERSION build --release

# build macos wheel (only builds for installed Python version)
pyo3-pack build --release

# upload built wheels to pypi
twine upload target/wheels/* --skip-existing
```
