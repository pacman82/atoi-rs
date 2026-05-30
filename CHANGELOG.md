# Changelog

## [3.0.0](https://github.com/pacman82/odbc-api/compare/2.0.0...3.0.0) - 2026-05-30

### 🚀 Features

- [**breaking**] Introduce FromHexDigit
- [**breaking**] Introduce from digit


### 🚜 Refactor

- [**breaking**] Builtin types implemented indpendent from Integer

  Builtin typen no longer implement MaxNumDigits

- [**breaking**] Minimal supported rust compiler now 1.94.1


### 📚 Documentation

- Document Benchmarking with native CPU instructions enabled


### ⚡ Performance

- Inline atoi
- Inline from_radix_* functions for builtin types

  Since these are no longer generic, we do not get the inlining for free.
  Benchmarks would show significant regressions otherwise

- Speed up hexdigit parsing by using masks to unify ascii cases

  Thanks to okaneco for the idea



## 2.0.0

* Minimal supported compiler is Rust 1.57.0
* Support for `no-std` by making `std` a default feature which can be opted out of.

## 1.0.0

* Minimal supported compiler is Rust 1.56.0
* Changed Rust edition to 2021
* Stabilized interface

## 0.4.0

* The `atoi` function now supports parsing signed integers. Use the `FromRadix10` trait directly if
  you wish to not allow leading `+` or `-` signs.

## 0.3.3

* Introduce `FromRadix10Signed` and `FromRadix10SignedChecked` for parsing signed integers.

## 0.3.2

* Add support for hex numbers through `FromRadix16` and `FromRadix16Checked`.
* Fix: Documentation of `FromRadix10Checked` now has code samples using this trait.

## 0.3.1

* Fix: Fixed documentation of `atoi`s overflow behaviour.

## 0.3.0

* Added `From_radix_10_checked`.
* Breaking change: atoi now returns `None` on overflow

## 0.2.4

* Documentation now hints at `FromRadix10` trait.
* Updated to Rust 2018
