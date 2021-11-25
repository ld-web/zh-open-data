# Chinese (中文) characters Open Data API

[![Build Status](https://app.travis-ci.com/ld-web/zh-open-data.svg?branch=master)](https://app.travis-ci.com/ld-web/zh-open-data)

This API aims to provide chinese characters informations, from a given input of one or more characters.

- [Usage](#usage)
- [Provided informations](#provided-informations)
- [File loading](#file-loading)
- [CLI usage](#cli-usage)
- [Documentation](#documentation)
- [Docker](#docker)

## Usage

There is a single endpoint : `/lookup`

Provide any character or series of characters you want after : `/lookup/你是哪國人`

## Provided informations

- Character
- CNS (Chinese National Standard) code
- Components set(s)
- Phonetic(s)
- Stroke informations
  - Total number of strokes
  - _in the future, stroke sequence too_

## File loading

### File names

At this time, the tool will look into the loading directory and attempt to load the following files at runtime :

- `load-dir`/CNS2UNICODE_Unicode_2.txt
- `load-dir`/CNS2UNICODE_Unicode_15.txt
- `load-dir`/CNS2UNICODE_Unicode_BMP.txt
- `load-dir`/CNS_component.txt
- `load-dir`/CNS_phonetic.txt
- `load-dir`/CNS_stroke.txt

### Loading directory (`load-dir` option)

By default, the tool will look into a `data` directory located at the root of the project.

It can be changed by using the `-l` or `--load-dir` option.

So if you want to change the directory where the tool will look, create and put the appropriate files into it.

> You can find a ZIP archive of Chinese characters Open Data [here](https://data.gov.tw/dataset/5961)

## CLI usage

If you want to use the binary as a CLI tool, checkout the [CLI-v1.0 tag](https://github.com/ld-web/zh-open-data/tree/cli-v1.0).

```bash
# Using cargo
cargo run 我是
cargo run -- -l /your/custom/directory 我是

# Using compiled binary
/path/to/zh_open_data 我是
/path/to/zh_open_data -l /your/custom/directory 我是

```

## Documentation

The documentation is auto-generated and deployed on [Github Pages](https://ld-web.github.io/zh-open-data/).

## Docker

A public image is available :

```sh
docker pull ghcr.io/ld-web/zh-open-data:latest
```

To run an instance, map a local port to the corresponding Docker exposed port (right now `8900`, in the future may become a CLI option), and create a volume between the folder containing the actual Open Data files and the container's `data` directory, where the application loads the files.

### Example

```sh
docker run -d --name zh_open_data -v /path/to/data:/app/data -p 8900:8900 ghcr.io/ld-web/zh-open-data
```
