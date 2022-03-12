# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Upgraded to `twilight-model` 0.10.0

## [0.9.1] - 2022-03-09
### Changed
- Implemented `Attachment` command option type. (by @Lyssieth)
- `command::internal` is now hidden from the documentation.

## [0.9.0] - 2022-01-24
### Changed
- Updated to `twilight-model` 0.9.
- Updated MSRV to 1.57 and to Edition 2021.

## [0.8.1] - 2021-12-21
### Added
- A `value` method is generated when deriving `CommandOption` for command option choices.
- `CreateCommand::NAME` associated constant to get the name of the command.
- `CommandInputData::parse_field` method to directly parse a field without command model.
- `CommandInputData::focused` method to get the name of the focused field.
- Implementation of `CommandModel` for `Vec<CommandDataOption>` and `CommandOption` for `CommandOptionValue`.

### Changed
- `CommandModel` and `CreateCommand` can be derived on unit structs.
- Improved validation of command names.

### Fixed
- `ParseError::EmptyOption` is only returned when parsing subcommands.  
   This fixes command models without options or with only optional options.

## [0.8.0] - 2021-12-12
### Added
- Subcommands and subcommand groups are supported by `CommandModel` and `CreateCommand` macros.
- Command option settings like `max_value` are validated when parsing command.

### Changed
- Updated to `twilight-model` 0.8.0.
- `CommandModel::from_interaction` now takes a `CommandInputData`.
- Internal types have been moved to a separate module.
- Improved documentation.

### Removed
- `http` feature has been removed.

## [0.7.2] - 2021-11-23
### Added
- New `autocomplete`, `max_value` and `min_value` on `CreateCommand` derive macro.

### Changed
- Updated to `twilight-model` 0.7.2.

## [0.7.1] - 2021-11-10
### Added
- Support of command option choices with the `CommandOption` and `CreateOption` traits.
- A dummy implementation is generated in case of macro error to avoid additional "unimplemented trait" compilation errors.

### Changed
- `ApplicationCommandData` can be converted into a twilight `Command` using `From`.

## [0.7.0] - 2021-10-28
### Added
- Initial release of `twilight-interactions` and `twilight-interactions-derive` crates.

[Unreleased]: https://github.com/baptiste0928/twilight-interactions/compare/v0.9.1...main
[0.9.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/baptiste0928/twilight-interactions/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/baptiste0928/twilight-interactions/releases/tag/v0.7.0
