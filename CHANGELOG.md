# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - No date specified

### Added

- `Reverb::process` function to process a block of samples.
- `Reverb::process_add` function to process a block of samples with mixing.

### Changed

- Made crate `no_std` for use in embedded context.
- Migrated to 2021 edition.
- Replaced `Reverb::construct` by `Default` implementation.
- Use GitHub Actions instead of Travis CI.

### Removed

- `dsp_node` module and example because of `dsp-chain` being unmaintained with no working release version.
