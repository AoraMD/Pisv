# Pisv

<img alt="License" src="https://img.shields.io/github/license/AoraMD/Pisv?style=flat-square"><img alt="Workflow" src="https://img.shields.io/github/workflow/status/AoraMD/Pisv/build?style=flat-square">

Pixiv illustration downloader.

> translated: [🇨🇳](doc/readme-cn.md)

## Usage

Pixiv now uses OAuth as login verification method that we cannot log in by using username and password directly. Please see [this page](./doc/login.md) to learn how to log in.

- Download illustrations in you private like collection into `~/illust`.

    ```
    > pisv like --scope private --path ~/illust
    ```

- Incrementally download illustration drawn by artist which ID is `123456`. The program will stop immediately when find a illustration has been downloaded.

    ```
    > pisv artist 123456 --increment
    ```

## To-Do

- [ ] Use domain fronting.

## License

```
MIT License

Copyright (c) 2022 M.D.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

