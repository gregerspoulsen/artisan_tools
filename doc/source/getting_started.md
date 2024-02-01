# Getting Started

Artisan Tools is a collection tools aimed at simplifying automation of
typical workflows in software development.

## Installation

Artisan tools can be installed directly from source with PIP:

.. code-block:: bash

    pip install git+https://github.com/gregerspoulsen/artisan_tools.git@container_arch

After installation the `at` command should be available in the shell.

## Usage

To use artisan tools create a file called `artisan.yml` in the root of the
project repository. The file can be left empty in which case the [default
configuration](config) is used.

Create a file containing the current version of the project. Default is a file
called `VERSION` in the root of project.

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
