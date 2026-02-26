# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-26

### Added

- `--strict` mode that errors on undefined template variables ([#2](https://github.com/jtdowney/rsubst/issues/2) — thanks @mcandre)
- `--help` flag for usage information

## [0.1.4] - 2025-04-02

### Changed

- Removed `anyhow` dependency
- Updated documentation with Dockerfile example and readme badges

## [0.1.3] - 2025-03-31

### Added

- Support for reading templates from stdin when no file argument is given

## [0.1.2] - 2025-03-31

### Changed

- Stripped symbols in release builds for smaller binaries
- Updated minijinja dependency

## [0.1.1] - 2025-03-31

### Added

- GitHub Actions release workflow
- Integration tests and CI configuration

## [0.1.0] - 2025-03-30

### Added

- Initial release
- Jinja2 template rendering with environment variables
- `split` filter for processing delimited values

[0.2.0]: https://github.com/jtdowney/rsubst/releases/tag/v0.2.0
[0.1.4]: https://github.com/jtdowney/rsubst/releases/tag/v0.1.4
[0.1.3]: https://github.com/jtdowney/rsubst/releases/tag/v0.1.3
[0.1.2]: https://github.com/jtdowney/rsubst/releases/tag/v0.1.2
[0.1.1]: https://github.com/jtdowney/rsubst/releases/tag/v0.1.1
[0.1.0]: https://github.com/jtdowney/rsubst/releases/tag/v0.1.0
