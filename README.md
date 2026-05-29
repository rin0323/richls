# richls
[![License](https://img.shields.io/badge/License-MIT-blue)](https://github.com/rin0323/richls_2026_Empirical/blob/main/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/rin0323/richls/badge.svg?branch=main)](https://coveralls.io/github/rin0323/richls?branch=main)

様々な機能を追加したls

# Discryption
`richls` は、標準的な `ls` コマンドを拡張したファイル一覧表示ツールです。

通常のファイル一覧表示に加えて、`-l, --long` を指定した場合は、標準的な `ls -l` に近い詳細情報を表示します。具体的には、ファイル種別とパーミッション、ハードリンク数、所有者、グループ、ファイルサイズ、最終更新日時、ファイル名を表示します。

さらに、`richls` では `-l, --long` 指定時に、ファイルサイズを `1.2KB` や `1.2GB` などの human readable 形式で表示します。また、ディレクトリ内に `README.md` が存在する場合はその概要を tagline として表示し、PDF ファイルの場合は PDF 内に埋め込まれたタイトルを表示します。作成日時が24時間以内のファイルには `🆕` を付与します。

必要に応じて、隠しファイルの表示、`.gitignore` や `.dockerignore` を考慮した表示、ファイルサイズ・最終更新日時・ファイル名によるソートにも対応します。

# Usage

```bash

Usage:
  richls [FILE]
  richls -l [OPTIONS] [FILE]

Argument:
  [FILE] 表示対象のパス。省略した場合は現在のディレクトリを表示します。

Options:
-l, --long
        詳細表示モードで出力する。
        ls -l と同様に、パーミッション、リンク数、所有者、グループ、
        ファイルサイズ、最終更新日時、ファイル名を表示する。
        また、ファイルサイズは human readable 形式で表示し、
        README tagline、PDF タイトル、🆕 マークも標準で表示する。

-a, --all
        . で始まる隠しファイルも表示する。

--respect-ignore
        .gitignore や .dockerignore を考慮して表示する。

--sort <key>
        ソート順を指定する。
        [name | size | mtime]

-h, --help
        ヘルプを表示する。

-V, --version
        バージョンを表示する。

$ richls
# 現在のディレクトリにあるファイルを表示
$ richls documents/
# documentディレクトリにあるファイルを表示
```
# Installasion

# About
## License

## Author
Yamaguchi Rin


