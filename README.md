rust-gist
=========

Yet another Gist publishing client.

## How to install

If you have cargo installed, you can install via cargo:

```
$ cargo install rust-gist
```

Otherwise, grab a release tarball with binary.

## How to use

1. Grab a personal token from [Github settings page](https://github.com/settings/tokens). Then run `rust-gist token YOUR_TOKEN`.
This would save token to `~/.config/rust-gist` directory in plain text.

2. After that, just run `rust-gist filename.txt` to upload it to Github Gist (you'll get a link to uploaded file back.

## Pro tips

### Stdin redirection

You can pipe text to rust-gist and it would upload that text to the gist.github.com:

```
$ tail longlog.txt | rust-gist
```

### Copying to clipboard

On Mac OS, add `| pbcopy` to automatically copy URL to the clipboard:

```
$ rust-gist some-file.js | pbcopy
```

## License

rust-git was written in 2017 by Tim Marinin and is licensed under terms of MIT license.
