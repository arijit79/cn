# CHANGELOG

## v2.0.0
### Added
* Symlinking of paths rather than copying
* Hard linking
* Bunch of new flags like
    * `-T`
    * `-H`
    * `-L`

### Changed
* All IO-bound operations are now asynchronous
* Format of errors which were not handled by checks function

### Deprecated
* The `--move` or `-m` flag is now soft-deprecated

## v1.0.1
### Fixed
* Fix critical bug when directories won't get copied with a `InvalidInput` error message
### Added
* Another check to see if source is not the same as destination
### Changed
* Use OsStr for reading a directory instead of converting into &str

## v1.0.0
* Added all features
* Fixed all bugs