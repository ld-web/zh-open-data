# Chinese (中文) characters Open Data

This tool aims to provide chinese characters informations, from a given input of one or more characters.

> Right now it works as a CLI tool, in the future it will be an API giving data as serialized JSON objects

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

- `loading_dir`/CNS2UNICODE_Unicode_2.txt
- `loading_dir`/CNS2UNICODE_Unicode_15.txt
- `loading_dir`/CNS2UNICODE_Unicode_BMP.txt
- `loading_dir`/CNS_component.txt
- `loading_dir`/CNS_phonetic.txt
- `loading_dir`/CNS_stroke.txt

### Loading directory (`load-dir` option)

By default, the tool will look into a `data` directory located at the root of the project.

It can be changed by using the `-l` or `--load-dir` option.

So if you want to change the directory where the tool will look, create and put the appropriate files into it.

> You can find a ZIP archive of Chinese characters Open Data [here](https://data.gov.tw/dataset/5961)

## Example usage

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