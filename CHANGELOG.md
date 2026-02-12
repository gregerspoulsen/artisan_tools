# Changelog

## [1.1.10] - 2026-02-12

- Add typing_extensions to explicit dependencies

## [1.1.9] - 2025-05-28

- Change changelog template to yaml format

## [1.1.8] - 2025-05-21

- Add template generation code
- Split main.rs to run.rs and cli.rs
- Refactor test crate for modularity

## [1.1.7] - 2025-05-21

- Add colored CLI style

## [1.1.6] - 2025-05-19

- Add diagrams for future workflow

## [1.1.5] - 2025-05-19

- Add osv-scanner to CI
- Add linting (Rust) and typo checking to CI

## [1.1.4] - 2025-05-14

- Create nix development environment

## [1.1.3] - 2025-05-14

- Add git info retrieval to rust version

## [1.1.2] - 2025-01-10

- Create structure for rustification

## [1.1.1] - 2024-10-08

- Fix errors not propagating from container.build_push

## [1.1.0] - 2024-09-13

- Add support for underscores in branch names

## [1.0.1] - 2024-05-16

- Fix incorrect version reported with `at --version`

## [1.0.0] - 2024-05-14

## Changed

- Use two files for versioning. `RELEASE` is next intended release, `VERSION`
  is automatically generated using `at version update`. To upgrade rename
  `VERSION` to `RELEASE` and make sure all builds (or other calls that rely on
  the `VERSION` file) are prepended with a call to `at version update`.

## [0.15.0] - 2024-04-16

### Added

- Support for --version

### Fixed

- Fix error when `build_push` is called with no tags
