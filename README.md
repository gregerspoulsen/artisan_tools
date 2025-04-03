# Artisan Tools
Tools to help the software artisan make his craft a breeze.

## Installation

Artisan Tools can be installed from source using pip:
```shell
pip install git+https://github.com/gregerspoulsen/artisan_tools.git
```

| :zap:        CLI only works if installed with pip>23.0   |
|----------------------------------------------------------|


## Usage

To use artisan tools in your project create a structure like:
```
project/
  + artisan.yaml
  + RELEASE
```
`artisan.yaml` can in most cases be left empty when default settings are
sufficient. `RELEASE` should contain the expected release version of the project
(e.g. 1.3.5).

For more information have a look at
[https://readthedocs.org/projects/artisan-tools/](https://artisan-tools.readthedocs.io/).


## Development
Ordinary tasks can be performed using just configured in justfile.

Before committing and pushing you should run:

```shell
just pre-commit
```
This will run code formatting linting and tests. To run a specific step like
testing use:
```shell
just test
```

To force a re-build of the container (e.g. for dependency changes):
```shell
just env
```

To run a command in the development environment (in this case `bash`):
```shell
just run bash
```

### Documentation
To build the documentation:
```shell
just doc
```

### Releasing

The pipeline automatically checks pull requests for unique and proper semver
version. To update the version when preparing a pull-request use
```shell
just bump <major|minor|patch>
```
When the pull-request is merged to master the release will automatically be
tagged with the version in the file VERSION.


## Rust Port Notes

just build-rs
just run-rs version update
just run-rs version get