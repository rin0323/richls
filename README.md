# richls

[![License](https://img.shields.io/badge/License-MIT-blue)](https://github.com/rin0323/richls_2026_Empirical/blob/main/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/rin0323/richls/badge.svg?branch=main)](https://coveralls.io/github/rin0323/richls?branch=main)

`richls` は、標準的な `ls` を拡張したファイル一覧表示ツールです。

通常の一覧表示に加えて、`-l, --long` を指定すると `ls -l` に近い詳細情報と、READMEの概要やPDFタイトルなどの補足情報を表示します。

## 機能

- ファイルまたはディレクトリ直下の一覧表示
- `-l, --long` による詳細表示
- `-a, --all` による隠しファイル表示
- `.gitignore` と `.dockerignore` の基本的なパターンを考慮した除外
- 名前、サイズ、最終更新日時によるソート
- Bash、Elvish、Fish、PowerShell、Zsh用の補完ファイル生成

## 詳細表示

`-l, --long` では、次の情報を表示します。

```text
MARK  MODE  LINKS  OWNER  GROUP  SIZE  MTIME  NAME  INFO
```

- `MARK`: 作成日時が24時間以内なら `new`。作成日時を取得できない場合は最終更新日時を使用
- `MODE`: ファイル種別とパーミッション
- `LINKS`: ハードリンク数
- `OWNER`: 所有者
- `GROUP`: グループ
- `SIZE`: 1024を基準にした `B`, `KB`, `MB`, `GB` などのhuman-readable表記
- `MTIME`: ローカル時刻の `YYYY-MM-DD HH:MM` 形式
- `NAME`: ファイル名。ディレクトリ名には `/` を付与
- `INFO`: ディレクトリのREADME概要、またはPDFタイトル

表示例:

```text
new  -rw-r--r--  1 rin staff    1.2KB 2026-07-06 12:00 main.rs
     drwxr-xr-x  5 rin staff    4.0KB 2026-07-05 10:12 docs/       README: Documentation
     -rw-r--r--  1 rin staff    1.2GB 2026-07-01 09:30 paper.pdf  PDF: Generic Malware Unpacking
```

READMEが存在しない場合やPDFタイトルを取得できない場合、`INFO` 欄は空になります。PDFタイトルは、PDF内で直接参照できる文字列形式または16進文字列形式の `/Title` を可能な範囲で読み取ります。圧縮されたメタデータなど、対応していない形式は安全に無視します。

## Usage

```text
Usage: richls [OPTIONS] [FILE]

Arguments:
  [FILE]  Path to list [default: .]

Options:
  -l, --long             Show ls -l style metadata and rich information
  -a, --all              Show hidden files
      --respect-ignore   Hide entries matched by .gitignore or .dockerignore
      --sort <KEY>       Sort by name, size, or mtime [default: name]
      --complete         Generate shell completion files
  -h, --help             Print help
  -V, --version          Print version
```

`--sort name` は名前の昇順、`--sort size` はサイズの大きい順、`--sort mtime` は更新日時の新しい順に表示します。

`--respect-ignore` は、表示対象ディレクトリ直下の項目に対して `.gitignore` と `.dockerignore` の基本的なパターンを適用します。否定ルールなど、gitignoreの全構文を再現するものではありません。

## Examples

```bash
# カレントディレクトリを表示
richls

# 隠しファイルを含めて詳細表示
richls -la

# ignoreファイルを考慮し、更新日時の新しい順に表示
richls -l --respect-ignore --sort mtime documents/

# シェル補完ファイルを completions/ に生成
richls --complete
```

## License

MIT

## Author

Yamaguchi Rin
