# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.1] - 2025-01-28
### Added
- `contexts` and `integration_types` attributes on `CreateCommand` (@fdnt7)

## [0.16.0] - 2025-01-17
### Changed
- Updated to `twilight-model` 0.16 (@randomairborne)

## [0.16.0-rc.1] - 2024-05-16
### Added
- A basic example bot implementation has been added in the `examples` directory
  of the repository.
- Localizations are now handled by the `DescLocalizations` and
  `NameLocalizations` structs.

### Changed
- Updated to `twilight-model` 0.16.0-rc.1 (@fdnt7)
- Localization functions now return `DescLocalizations` or `NameLocalizations`.
- `desc` should not be provided anymore if `desc_localizations` is provided.
- Various improvements to the documentation.
- Improved macro error messages.

## [0.15.2] - 2023-06-23
### Added
- Subcommands enums now support `Box`ed variants to avoid large enums.
- `ResolvedMentionable` type can be used to resolve a mentionable to either
  a user or a role.

### Fixed
- Strings are now trimmed in macro attributes to match Discord's behavior.

### Changed
- Bumped MSRV to 1.67.

## [0.15.1] - 2023-03-26
### Fixed
- Attribute parameters names are now correctly validated. (#21)

### Changed
- Updated to `syn` 2.
- Error messages are now in lowercase to match compiler errors.

## [0.15.0] - 2023-02-07
### Changed
- Updated to `twilight-model` 0.15.0. (@randomairborne)

## [0.14.4] - 2023-01-14
### Added
- `GuildDirectory` and `GuildForum` channel types. (@CircuitSacul)

## [0.14.3] - 2023-01-08
### Added
- Support age-restricted commands with the `nsfw` attribute.

### Fixed
- Fixed compilation errors with `twilight-model` 0.14.1 and above.

## [0.14.2] - 2022-11-27
### Fixed
- Use new `ChannelType` variant names.

## [0.14.1] - 2022-11-21
### Fixed
- Set `required` to `None` on subcommand and subcommand group options.

## [0.14.0] - 2022-11-16
### Changed
- Upgraded to `twilight-model` 0.14.
- MSRV bumped to 1.64.

## [0.13.0] - 2022-08-15
### Changed
- Upgraded to `twilight-model` 0.13.
- Most types exported by this crate do not longer implement `Eq`.

### Removed
- `Number` type has been removed in twilight-model, use `f64` instead.

## [0.12.0] - 2022-07-17
### Added
- Support for autocomplete interactions with `AutocompleteValue`.
- Added `max_length` and `max_length` attributes for `String` fields.

### Changed
- Upgraded to `twilight-model` 0.12

## [0.11.0] - 2022-05-16
### Added
- Support command localization with `name_localizations` and `desc_localizations` attributes.
- Command permissions v2 with `default_permissions` and `dm_permission` attributes.

### Changed
- Upgraded to `twilight-model` 0.11.0
- MSRV updated to 1.60

## [0.10.1] - 2022-03-15
### Added
- Allow `CommandModel` and `CreateCommand` types to have generics (@MaxOhn)
- Implement `CommandOption` & `CreateOption` for `Cow<'_, str>` (@MaxOhn)

## [0.10.0] - 2022-03-13
### Changed
- Upgraded to `twilight-model` 0.10.0

### Removed
- Autocomplete interactions parsing has been removed. See [#9](https://github.com/baptiste0928/twilight-interactions/issues/9).

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

[Unreleased]: https://github.com/baptiste0928/twilight-interactions/compare/v0.16.1...main
[0.16.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.16.0...v0.16.1
[0.16.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.16.0-rc.1...v0.16.0
[0.16.0-rc.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.15.2...v0.16.0-rc.1
[0.15.2]: https://github.com/baptiste0928/twilight-interactions/compare/v0.15.1...v0.15.2
[0.15.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.14.4...v0.15.0
[0.14.4]: https://github.com/baptiste0928/twilight-interactions/compare/v0.14.3...v0.14.4
[0.14.3]: https://github.com/baptiste0928/twilight-interactions/compare/v0.14.2...v0.14.3
[0.14.2]: https://github.com/baptiste0928/twilight-interactions/compare/v0.14.1...v0.14.2
[0.14.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/baptiste0928/twilight-interactions/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/baptiste0928/twilight-interactions/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/baptiste0928/twilight-interactions/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/baptiste0928/twilight-interactions/releases/tag/v0.7.0
