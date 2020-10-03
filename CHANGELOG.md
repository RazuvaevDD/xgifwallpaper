# Changelog

## [Unreleased](https://github.com/calculon102/xgifwallpaper/tree/HEAD)

[Full Changelog](https://github.com/calculon102/xgifwallpaper/compare/v0.2.0...master)

## [v0.2.0](https://github.com/calculon102/xgifwallpaper/tree/v0.2.0) - 2020-10-04

[Full Changelog](https://github.com/calculon102/xgifwallpaper/compare/v0.1.2...v0.2.0)

### Added

- This changelog
- Option `-s` to scale GIF to `FILL` screen or `MAX`-out as much as possible
- Sample GIFs and run-script as starter for semi-automated integration-tests

### Fixed

- Set background of root window to black on exit
- Exit gracefully, if there is no X display to open
- Exit gracefully, if given file is not a valid GIF


## [v0.1.2](https://github.com/calculon102/xgifwallpaper/tree/v0.1.2) - 2020-09-04

[Full Changelog](https://github.com/calculon102/xgifwallpaper/compare/v0.1.1...v0.1.2)

### Fixed

- Compositors get segmentation fault after closing program [\#2](https://github.com/calculon102/xgifwallpaper/issues/2)


## [v0.1.1](https://github.com/calculon102/xgifwallpaper/tree/v0.1.1) - 2020-09-03

[Full Changelog](https://github.com/calculon102/xgifwallpaper/compare/v0.1.0...v0.1.1)

### Fixed

- Colors are not respected [\#1](https://github.com/calculon102/xgifwallpaper/issues/1)

## [v0.1.0](https://github.com/calculon102/xgifwallpaper/tree/v0.1.0) - 2020-08-29

[Full Changelog](https://github.com/calculon102/xgifwallpaper/compare/3b85a0131b52672b3f5c82d7d721b9a7c4da9769...v0.1.0)

### Added

- Animate GIF as background on root-window of a X-session
- Use `-b` to customize background-color for transparent or non-image pixels
- Use `-d` to specifiy a default-delay between frames, if none specified
- Use `-v` to be verbose about it