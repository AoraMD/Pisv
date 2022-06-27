# Pisv

<img alt="License" src="https://img.shields.io/github/license/AoraMD/Pisv?style=flat-square">

Pixiv 插画下载工具

## 使用

Pixiv 现在采用 OAuth 鉴权所以无法直接使用账号密码登录，请查看 [这个页面](./login-cn.md) 了解如何登录。

- 将你私密收藏的插画下载到 `~/illust` 文件夹中

    ```
    > pisv like --scope private --path ~/illust
    ```

- 增量下载 ID 为 `123456` 画师创作的插画，程序在找到已下载过的插画时会停止运行

    ```
    > pisv artist 123456 --increment
    ```

## 待添加的功能

- [ ] 使用域前置

## 许可协议

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

