# richls
[![License](https://img.shields.io/badge/License-MIT-blue)](https://github.com/rin0323/richls_2026_Empirical/blob/main/LICENSE)

様々な機能を追加したls

# Discryption
このツールは、従来の ls を拡張し、ファイルの情報をより分かりやすく表示します。
ファイルサイズの人間可読化や、README・PDFのメタデータ表示などに対応しています。
また、更新が新しいファイルには"new"を付与し、.gitignore 等も考慮して表示します。
用途に応じてソート方法を柔軟に変更可能です。

# Usage

```bash

Usage:
  richls [FILE]
  richls -l [OPTIONS] [FILE]

Argument:
  [FILE] 表示対象のパス。省略した場合は現在のディレクトリを表示します。

Options:
  -l, --long              詳細表示モード

      --humanize          ファイルサイズを 1.2GB などの形式で表示
      --tagline           README.md の概要を表示
      --pdf-title         PDF 内のタイトルを表示
      --respect-ignore    .gitignore などを考慮して表示
      --new-mark          24時間以内のファイルに"new"を付与
      --sort <key>        ソート順を指定
                          [name | size | mtime]

  -h, --help              ヘルプを表示

  -V, --version           バージョンを表示

$ richls
# 現在のディレクトリにあるファイルを表示
$ richls documents/
# documentディレクトリにあるファイルを表示
```
