# Artisan Tools
Tools to help the software artisan make his craft a breeze.


## Development
Ordinary tasks can be used using go-task configured in taskfile.yaml.

Before committing and pushing you should run:

```shell
task pre-commit
```
This will run code formatting linting and tests. To run a specific step like
testing use:
```shell
task test
```

To force a re-build of the container (e.g. for dependency changes):
```shell
task build
```

To run a command in the development environment (in this case `bash`):
```shell
task run -- bash
```

## Releasing

The pipeline automatically checks pull requests for unique and proper semver
version. To update the version when preparing a pull-request use
```shell
task bump -- major|minor|patch
```
When the pull-request is merged to master the release will automatically be
tagged with the version in the file VERSION.


```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```