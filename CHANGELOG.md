# Changelog

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
