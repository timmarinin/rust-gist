# Changelog

This is a changelog for `rust-gist`.

## 0.1.3 (2017-10-10)

In this version I switched to four spaces, run the code through rustfmt and switched to handling
errors via error_chain crate.

- [x] use error_chain for error handling
- [x] add editorconfig

## 0.1.2 (2017-10-06)

Hotfix for 0.1.1: the behaviour was mixed up, it uploaded as anonymous without the flag,
and as a token owner with the flag.

Also the code was simplified leveraging the json! macro from serde_json.

- [x] hotfix --anonymous

## 0.1.1 (2017-10-06)

Minor feature release.

- [x] support --anonymous flag to upload without a token

## 0.1.0 (2017-10-05)

Initial public release. Submitted to lobste.rs and for impromtu code review to #rust@mozilla.

This release includes:

- [x] uploading single file to gist.github.com as private or public gist
- [x] saving token to file and reading token from file
