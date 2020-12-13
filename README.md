# markdown-html-finder [![PyPI](https://img.shields.io/pypi/v/markdown-html-finder.svg)](https://pypi.org/project/markdown-html-finder/)

A Python library to locate HTML spans in markdown text. This library is written in Rust with bindings for Python.

## why?
For a [separate project](https://github.com/chdsbd/kodiak) I needed to locate HTML comments in markdown documents. Sadly the markdown parsers I found for Python didn't provide span information for nodes.

While it wouldn't be too hard to add some features to existing Python markdown parsers, I thought it would be interesting to see how Rust can be used from Python. The excellent [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) crate provides span information for HTML elements, so that's what we use here.

[pyo3](https://github.com/PyO3/pyo3) and [maturin](https://github.com/PyO3/maturin) do the hard work of providing bindings to Python and building wheels to distribute on PyPi.

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
poetry run maturin development
```

### building wheels
We need a wheel per version and platform. To support Python 3.7, 3.8, 3.9 we need to have 3.7, 3.8, 3.9 installed on macOS and Linux. For macOS we can use pyenv. For Linux we can use a Docker container.

#### macos
1. install pyenv
2. install each python version we want to support via `pyenv install`. Use `pyenv install --list` to see the available options.
3. add your new Python installs globally via `pyenv global 3.8.7 3.9.0`
4. configure your $PATH with the .pyenv python versions. use `pyenv shims` to find the binary paths and add them, like `PATH=/Users/chris/.pyenv/shims/:$PATH`
5. verify your Python versions are accessible via `python3.9` and verify maturin can find your python versions via `./.venv/bin/maturin list-python`
6. build the macOS wheels via `./.venv/bin/maturin build`
7. upload wheels to pypi via `./.venv/bin/twine upload --skip-existing target/wheels/*`

#### linux
1. use the docker container to build all the Linux Python wheels via `docker run --rm -v $(pwd):/io cdignam/markdown-html-finder-builder:0.3.0 build --release`
2. upload wheels to pypi via `./.venv/bin/twine upload --skip-existing target/wheels/*`


### markdown-html-finder-builder
This container extends the [quay.io/pypa/manylinux2014_x86_64](https://quay.io/pypa/manylinux2014_x86_64) docker image and is based on the [konstin2/maturin](https://hub.docker.com/r/konstin2/maturin) image, with Python2 support removed.

This image is built and uploaded manually to Docker Hub when necessary.

```bash
# build and publish a new version
VERSION='0.2.0'
docker build -f build.Dockerfile . --tag cdignam/markdown-html-finder-builder:$VERSION
docker push cdignam/markdown-html-finder-builder:$VERSION
```
