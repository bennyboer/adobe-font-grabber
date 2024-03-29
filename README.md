# Adobe font grabber

Tool to copy all fonts installed via Adobe CC to a folder of your choice.

## What's New?

Hard-coded the data directory (`%appdata%` for Windows, `$HOME/Library/Application Support` for MacOS) to the program so you only need to put one argument which is specifically the save location of the fonts.

## Download

You can download a pre-built binary for 64-bit Windows on the GitHub releases page under "Assets": https://github.com/bennyboer/adobe-font-grabber/releases
If you do not use Windows you will have to built your own binary (See [Building](#building)).

## Usage

Use the tool on your command line as follows:

```sh
./adobe-font-grabber "<FOLDER TO COPY FONTS TO>"
```

## Example

I only tested the tool for Windows but it should probably also work for other operating systems. For Mac OS the path the
fonts are located in is something like `<HOME>/Library/Application/Support/Adobe/CoreSync/plugins/livetype/.r/`.

### Windows

```sh
./adobe-font-grabber.exe "C:/Users/<USERNAME>/Downloads/ADOBE_FONTS"
```

If everything went as planned the following message should appear once done:

```
Successfully copied X fonts to `C:/Users/<USERNAME>/Downloads/ADOBE_FONTS`
```

## Building

You will need to setup a Rust toolchain for you system if you haven't already (Check https://www.rust-lang.org/).
Afterwards just run `cargo build --release` and the built binary will appear under `target/release`.
