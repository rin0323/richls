# richls

[![GitHub Release](https://img.shields.io/github/v/release/rin0323/richls)](https://github.com/rin0323/richls/releases/latest)
[![build](https://github.com/rin0323/richls/actions/workflows/build.yaml/badge.svg)](https://github.com/rin0323/richls/actions/workflows/build.yaml)
[![License](https://img.shields.io/badge/License-MIT-blue)](https://github.com/rin0323/richls/blob/main/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/rin0323/richls/badge.svg?branch=main)](https://coveralls.io/github/rin0323/richls?branch=main)

## Tagline（1行概要）

`richls` は、通常の `ls` に読みやすい詳細情報と文脈情報を加える Rust 製ファイル一覧表示ツールです。

## 概要（Overview）

`richls` は、標準的な `ls` を拡張したファイル一覧表示ツールです。
通常の一覧表示に加えて、`-l, --long` を指定すると `ls -l` に近い詳細情報と、READMEの概要やPDFタイトルなどの補足情報を表示します。

主な機能は以下の通りです。

- ファイルまたはディレクトリ直下の一覧表示
- `-l, --long` による詳細表示
- `-a, --all` による隠しファイル表示
- `.gitignore` と `.dockerignore` の基本的なパターンを考慮した除外
- 名前、サイズ、最終更新日時によるソート
- `--clean-suggest` による削除候補ファイルの表示
- Bash、Elvish、Fish、PowerShell、Zsh用の補完ファイル生成

## 使い方（Usage）

```text
Usage: richls [OPTIONS] [FILE]

Arguments:
  [FILE]  Path to list [default: .]

Options:
  -l, --long             Show metadata, human-readable sizes, and rich information
  -a, --all              Show hidden files
      --respect-ignore   Hide entries matched by .gitignore or .dockerignore
      --sort <KEY>       Sort by name, size, or mtime [default: name]
      --complete         Generate shell completion files
      --clean-suggest    Suggest cleanup candidate files without deleting them
  -h, --help             Print help
  -V, --version          Print version
```

基本的な実行例です。

```bash
# カレントディレクトリを表示
richls

# 隠しファイルを含めて詳細表示
richls -la

# ignoreファイルを考慮し、更新日時の新しい順に表示
richls -l --respect-ignore --sort mtime documents/

# 削除候補になりそうな通常ファイルを表示
richls --clean-suggest

# シェル補完ファイルを completions/ に生成
richls --complete
```

`--sort name` は名前の昇順、`--sort size` はサイズの大きい順、`--sort mtime` は更新日時の新しい順に表示します。
`--respect-ignore` は、表示対象ディレクトリ直下の項目に対して `.gitignore` と `.dockerignore` の基本的なパターンを適用します。否定ルールなど、gitignoreの全構文を再現するものではありません。

### 詳細表示

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

### 削除候補の表示

`--clean-suggest` を指定すると、通常の `ls` では見落としやすい削除候補になりそうな通常ファイルを表示します。
この機能は候補を表示するだけで、実際にファイルを削除しません。削除確認プロンプトや `rm` 相当の処理も行いません。

削除候補として検出する条件は以下の通りです。

- `copy`, `コピー`, `のコピー`, `(1)` や `(10)` など、コピーされたファイルの可能性がある名前
- `old`, `backup`, `bak` など、古い版やバックアップの可能性がある名前
- ファイルサイズが 0B の通常ファイル
- 最終更新日時が現在時刻から 180 日以上前の通常ファイル
- `.tmp`, `.swp`, `~` で終わる一時ファイル

## インストール方法（Installation）

`richls` は Rust 製の CLI ツールです。事前に Rust と Cargo をインストールしてください。

```bash
rustc --version
cargo --version
```

ソースコードからインストールする場合は、次のように実行します。

```bash
git clone https://github.com/rin0323/richls.git
cd richls
cargo install --path .
```

インストールせずにビルド成果物を確認する場合は、release ビルドを実行します。

```bash
cargo build --release
./target/release/richls --help
```

将来 crates.io に公開した場合は、次の形式でインストールできます。

```bash
cargo install richls
```

## プロジェクトについて（About）

### 開発者

Yamaguchi Rin

### ライセンス

MIT License で公開しています。詳細は [LICENSE](LICENSE) を参照してください。

### 名前の由来

`richls` は、`ls` に richer な情報を加えるという意味から名付けています。単なるファイル名一覧ではなく、サイズ、更新日時、README概要、PDFタイトル、新規ファイル印などを合わせて確認できる一覧表示を目指しています。

### バージョン履歴

- `v0.1.0`: 初期リリース。基本的な一覧表示、詳細表示、ignore 対応、ソート、README/PDF 情報表示、削除候補表示を提供。

最新のリリースは [GitHub Releases](https://github.com/rin0323/richls/releases/latest) を確認してください。
