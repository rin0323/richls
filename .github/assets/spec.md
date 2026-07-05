# Specification of `richls`

## 1. Overview

`richls` は、標準的な `ls` コマンドを拡張したファイル一覧表示ツールである。

通常のファイル一覧表示に加えて、`-l, --long` 指定時には標準的な `ls -l` に近い詳細情報を表示する。さらに、ファイルサイズの human readable 表示、README の tagline 表示、PDF タイトル表示、24時間以内に作成されたファイルへの `new` マーク付与を行うことで、ファイルやディレクトリの内容をより把握しやすくする。

## 2. Input

入力は、ファイルまたはディレクトリのパスである。

パスが省略された場合は、カレントディレクトリを対象とする。

```bash
richls
richls documents/
richls README.md
```

## 3. Basic Behavior

入力されたパスがファイルである場合、そのファイルの情報を表示する。

入力されたパスがディレクトリである場合、そのディレクトリ内のファイルおよびディレクトリの一覧を表示する。

オプションを指定しない場合は、ファイル名およびディレクトリ名を表示する。

```text
Cargo.toml
README.md
src/
target/
```

## 4. Long Format

`-l` または `--long` オプションを指定した場合、詳細表示モードで出力する。

詳細表示モードでは、標準的な `ls -l` と同様に、各エントリについて以下の情報を表示する。

* ファイル種別およびパーミッション
* ハードリンク数
* 所有者
* グループ
* ファイルサイズ
* 最終更新日時
* ファイル名

また、`richls` では `-l, --long` 指定時に以下の付加情報を標準で表示する。

* ファイルサイズの human readable 表示
* ディレクトリ内の `README.md` の tagline 表示
* PDF ファイル内に埋め込まれたタイトル表示
* 作成日時が24時間以内のファイルへの `new` マーク付与

### 4.1 Output Columns

`-l, --long` 指定時の出力形式は以下とする。

```text
MARK  MODE  LINKS  OWNER  GROUP  SIZE  MTIME  NAME  INFO
```

各列の意味は以下の通りである。

| Column  | Description                      |
| ------- | -------------------------------- |
| `MARK`  | 作成日時が24時間以内の場合に `new` を表示する      |
| `MODE`  | ファイル種別およびパーミッション                 |
| `LINKS` | ハードリンク数                          |
| `OWNER` | 所有者                              |
| `GROUP` | グループ                             |
| `SIZE`  | human readable 形式のファイルサイズ        |
| `MTIME` | 最終更新日時                           |
| `NAME`  | ファイル名またはディレクトリ名                  |
| `INFO`  | README tagline や PDF タイトルなどの追加情報 |

表示例:

```text
new   -rw-r--r--  1  rin  staff  1.2KB  2026-05-29 14:20  main.rs
      drwxr-xr-x  5  rin  staff  4.0KB  2026-05-28 10:12  docs/       README: 実験資料まとめ
      -rw-r--r--  1  rin  staff  1.2GB  2026-05-20 09:30  paper.pdf   PDF: Generic Malware Unpacking
```

## 5. Standard Features in Long Format

### 5.1 Human Readable Size

`-l, --long` 指定時、ファイルサイズを1024を基準として `1.2KB`、`3.4MB`、`1.2GB` などの人間が読みやすい形式に変換して表示する。

最終更新日時はローカル時刻の `YYYY-MM-DD HH:MM` 形式で表示する。

### 5.2 README Tagline

`-l, --long` 指定時、対象がディレクトリであり、そのディレクトリ内に `README.md` が存在する場合、`README.md` の概要または先頭の有効な行を tagline として表示する。

tagline は `INFO` 欄に以下の形式で表示する。

```text
README: <tagline>
```

`README.md` が存在しない場合、または表示できる有効な行が存在しない場合は、tagline は表示しない。

### 5.3 PDF Title

`-l, --long` 指定時、対象が PDF ファイルである場合、PDF ファイルに埋め込まれたタイトルを表示する。

PDF タイトルは `INFO` 欄に以下の形式で表示する。

```text
PDF: <title>
```

PDF 内にタイトル情報が存在しない場合、タイトルを取得できない場合、またはタイトルが未対応の圧縮形式で格納されている場合は、PDF タイトルを表示しない。一覧処理自体は継続する。

### 5.4 New Mark

`-l, --long` 指定時、作成日時が24時間以内のファイルに `new` を付与して表示する。

`new` は `MARK` 欄に表示する。

```text
new   -rw-r--r--  1  rin  staff  1.2KB  2026-05-29 14:20  main.rs
```

作成日時を取得できない環境では、実装上の方針として最終更新日時を代替として用いてもよい。

## 6. Optional Features

以下の機能は、明示的にオプションを指定した場合に有効になる。

### 6.1 All Files

`-a` または `--all` オプションを指定した場合、`.` で始まる隠しファイルも表示する。

```bash
richls -a
richls -la
```

`-a, --all` は隠しファイルの表示を制御するオプションであり、`.gitignore` や `.dockerignore` の考慮とは別の機能である。

### 6.2 Respect Ignore

`--respect-ignore` オプションを指定した場合、`.gitignore` や `.dockerignore` を考慮して、不要なファイルやディレクトリを除外して表示する。

```bash
richls -l --respect-ignore
```

`--respect-ignore` は ignore ファイルに基づく除外を行うためのオプションである。`-a, --all` は隠しファイルを表示するオプションであるため、両者は別の意味を持つ。

### 6.3 Sort

`--sort <key>` オプションを指定した場合、出力順を変更する。

利用可能な値は以下である。

* `name`
* `size`
* `mtime`

```bash
richls -l --sort name
richls -l --sort size
richls -l --sort mtime
```

`name` を指定した場合、ファイル名順に並び替える。

`size` を指定した場合、ファイルサイズの大きい順に並び替える。

`mtime` を指定した場合、最終更新日時の新しい順に並び替える。

## 7. Options

```text
-l, --long
        詳細表示モードで出力する。
        ls -l と同様の情報として、パーミッション、リンク数、所有者、
        グループ、ファイルサイズ、最終更新日時、ファイル名を表示する。
        また、human readable なサイズ表示、README tagline 表示、
        PDF タイトル表示、new マーク表示を標準で行う。

-a, --all
        . で始まる隠しファイルも表示する。

--respect-ignore
        .gitignore や .dockerignore を考慮して表示する。

--sort <key>
        出力順を指定する。
        指定可能な値は name, size, mtime である。

--complete
        Bash、Elvish、Fish、PowerShell、Zsh 用の補完ファイルを
        completions/ ディレクトリへ生成する。

-h, --help
        ヘルプを表示する。

-V, --version
        バージョンを表示する。
```

## 8. Expected Use Cases

想定される利用場面は以下である。

* プロジェクトディレクトリの内容確認
* 標準的な `ls -l` に近い形式での詳細確認
* README の概要を含めたディレクトリ一覧の確認
* PDF ファイルのタイトル確認
* 新しく作成されたファイルの確認
* `.gitignore` や `.dockerignore` に従って不要なファイルを除外した一覧確認
* ファイルサイズや更新日時に基づく並び替え

## 9. Summary

本ツールの動作は、入力されたパスに基づいてファイル一覧を取得し、オプションに応じて表示内容を変更するものである。

`-l, --long` 指定時には、標準的な `ls -l` に近い詳細情報を表示し、さらに `richls` 独自の機能として、ファイルサイズの human readable 表示、README tagline 表示、PDF タイトル表示、24時間以内に作成されたファイルへの `new` マーク付与を標準で行う。

一方、隠しファイルの表示、ignore ファイルの考慮、ソート順の変更は、必要に応じて `-a, --all`、`--respect-ignore`、`--sort` オプションによって有効化する。
