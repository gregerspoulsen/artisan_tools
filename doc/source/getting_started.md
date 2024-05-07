# Getting Started

Artisan Tools is a collection tools aimed at simplifying automation of
typical workflows in software development.

## Installation

Artisan tools can be installed directly from source with PIP:

```bash
pip install git+https://github.com/gregerspoulsen/artisan_tools.git@container_arch
```

After installation the `at` command should be available in the shell.
`at --version` will display the current version of artisan-tools.

## Usage

To use artisan tools create a file called `artisan.yml` in the root of the
project repository. The file can be left empty in which case the [default
configuration](config) is used.

Versioning happens through two files `RELEASE` and `VERSION`. `RELEASE`
holds the next intended version and should be tracked by source control.
`VERSION` holds the current version, functionality using the version should read
from this file or register a hook for updates elsewhere. `VERSION` is generated
using the `at version update` command and should not be tracked by source control.
If `at version update` is run in non-release mode additional build information
is added.

With the two files `artisan.yml` and `RELEASE` you can start using the basic
functionality in artisan-tools.

Functionality in artisan-tools comes through extensions, explore the available
commands with `at --help`. The standard extensions
(`version`, `container`, `vcs`) are loaded automatically, Additional extensions
can be specified in the [configuration](config).

To get more information about the `version` command type `at version --help`.
This will show that you can bump the minor version using the command
`at version bump minor`. Get more information about the bump command using
`at version bump --help`. To get an overview of the available functionality
have a look at [CLI documentation](cli).

For more inspiration have a look in the `justfile` to see how artisan tools
is utilized to automate workflows internally.
