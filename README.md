# packman

[![Coverage Status](https://coveralls.io/repos/github/Rx-K8/packman/badge.svg?branch=develop)](https://coveralls.io/github/Rx-K8/packman?branch=develop)

A tool for archiving files and directories and extracting several archive formats.

## Description
Many archive fromats and their tools exist.
Each tool has a different instarface and is often used in a different way.
packman allows you to quickly extract archive files without having to worry about the interface of the archive file.
It is a CLI (Command Line Interface) tool that can handle various compression formats with a unified interface.

## Usage

```sh
A tool for archiving files and directories and extracting several archive formats.

Usage: packman [OPTIONS] [ARGUMENTS]...

Arguments:
  [ARGUMENTS]...  List of files or directories to be processed.

Options:
  -m, --mode <MODE>         Mode of operation. [default: auto] [possible values: auto, archive, extract, list]
  -o, --output <DEST>       Output file in archive mode, or output directory in extraction mode
  -e, --encrypt             Password for encrypted archives (archive mode).
  -r, --recursive           Recurse into directories (archive mode).
      --overwrite           Overwrite existing files.
  -h, --help                Print help
  -V, --version             Print version
```

## About

### Authors
* Keito Fukuoka ([Rx-K8](https://github.com/Rx-K8))

### The Logo
![logo](site/assets/logo.jpeg)

