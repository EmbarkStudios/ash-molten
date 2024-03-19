<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.18.0] - 2024-03-19
### Changed
- [PR#82](https://github.com/EmbarkStudios/ash-molten/pull/82) Upgrade MoltenVK to `1.2.8`

## [0.17.0] - 2024-01-24
### Changed
- [PR#81](https://github.com/EmbarkStudios/ash-molten/pull/81) Upgrade MoltenVK to `1.2.7`

## [0.16.0] - 2023-11-03
### Changed
- [PR#76](https://github.com/EmbarkStudios/ash-molten/pull/76) Upgrade MoltenVK to `1.2.6`

## [0.15.0] - 2023-03-15
### Changed
- [PR#75](https://github.com/EmbarkStudios/ash-molten/pull/75) Upgrade MoltenVK to `1.2.2`

## [0.14.0] - 2022-11-16
### Added
- [PR#73](https://github.com/EmbarkStudios/ash-molten/pull/73) added the `v1_1_10` and `v1_1_5` features, enabling one of these features will download or compile that version of MoltenVK instead of the version that is hardcoded in the current build script. This can be useful if a newer version of this crate uses a newer version of MoltenVK, but you want to use an older version due to reasons such as bugs or other problems with the newer version.

### Fixed
- [PR#73](https://github.com/EmbarkStudios/ash-molten/pull/73) changed the build script so that `ash-molten` can be cross-compiled from non-macOS hosts, at least in the pre-compiled case, the compiled case is untested.
- [PR#74](https://github.com/EmbarkStudios/ash-molten/pull/74) resolved [#70](https://github.com/EmbarkStudios/ash-molten/issues/70) by making the build script fail if the target os is not `macos` or `ios`.

## [0.13.1] - 2022-10-04
### Changed
- Expanded ash version range up to 0.37

## [0.13.0] - 2022-06-08
### Changed
- Update to MoltenVK 1.1.10

## [0.11.0] - 2021-10-14
### Added
- Provide entrypoint through Ash loader

## [0.10.0] - 2021-08-31
### Changed
- Update to MoltenVK 1.1.5
- Other stuff no one bothered to write in a CHANGELOG

## [0.7.2] - 2020-11-13
### Added
- Added prebuilt libs

## [0.7.1] - 2020-11-10
### Fixed
- Disabled debug build of MoltenVK

## [0.7.0] - 2020-11-09
### Changed
- Update to MoltenVK 1.1.0

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/ash-molten/compare/0.17.0...HEAD
[0.17.0]: https://github.com/EmbarkStudios/ash-molten/compare/0.16.0...0.17.0
[0.16.0]: https://github.com/EmbarkStudios/ash-molten/compare/0.15.0...0.16.0
[0.15.0]: https://github.com/EmbarkStudios/ash-molten/compare/0.14.0...0.15.0
[0.14.0]: https://github.com/EmbarkStudios/ash-molten/compare/v0.13.1+1.1.10...0.14.0
[0.13.1]: https://github.com/EmbarkStudios/ash-molten/compare/v0.13.0+1.1.10...v0.13.1+1.1.10
[0.13.0]: https://github.com/EmbarkStudios/ash-molten/compare/v0.11.0+1.1.5...v0.13.0+1.1.10
[0.11.0]: https://github.com/EmbarkStudios/ash-molten/compare/v0.10.0...v0.11.0+1.1.5
[0.10.0]: https://github.com/EmbarkStudios/ash-molten/compare/v0.7.2...v0.10.0
[0.7.2]: https://github.com/EmbarkStudios/ash-molten/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/EmbarkStudios/ash-molten/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/EmbarkStudios/ash-molten/releases/tag/v0.7.0
