---
title: "richls Usage"
date: 2026-06-26T00:00:00+09:00
draft: false
summary: "richls の基本的な使い方と、よく使うオプションの組み合わせを紹介します。"
description: "richls の実行方法、パス指定、詳細表示、README や PDF 情報の確認方法を説明します。"
tags: ["richls", "usage", "cli"]
categories: ["Guide"]
ShowToc: true
TocOpen: true
ShowReadingTime: true
ShowBreadCrumbs: true
ShowPostNavLinks: true
---

`richls` は、指定されたパスを読み取り、オプションに応じて情報を加工し、結果を標準出力へ表示します。
通常の `ls` と同じように使い始められ、必要に応じて詳細情報や文脈情報を足していけます。

## カレントディレクトリを表示する

```bash
richls
```

パスを省略すると、現在のディレクトリを対象にします。

## パスを指定する

```bash
richls ./src
richls ./docs
richls README.md
```

対象には、ファイルまたはディレクトリのパスを指定できます。

## 詳細表示を使う

```bash
richls -l .
richls --long .
```

`-l` または `--long` を指定すると、詳細表示モードになります。
詳細表示では、ファイルサイズ、更新日時、権限などのメタ情報に加えて、`richls` が取得した補助情報を表示します。

## 読みやすいサイズで見る

```bash
richls -l --humanize ./assets
```

`--humanize` は、ファイルサイズを KB / MB / GB などの形式で読みやすく表示するためのオプションです。
現行実装では、`--humanize` は詳細表示を有効にする互換オプションとして扱われます。

## README の概要を見る

```bash
richls --tagline ./packages
```

`--tagline` を指定すると、ディレクトリ内に `README.md` が存在する場合、その概要や先頭行を表示します。
パッケージやサブプロジェクトが並ぶディレクトリで、各項目の役割をすばやく把握できます。

## PDF のタイトルを見る

```bash
richls --pdf-title ./papers
```

`--pdf-title` を指定すると、PDF ファイルに埋め込まれたタイトルを表示します。
ファイル名だけでは内容が分かりにくい資料、論文、仕様書の確認に向いています。

## ignore ファイルを尊重する

```bash
richls --respect-ignore .
```

`--respect-ignore` を指定すると、`.gitignore` や `.dockerignore` の内容を考慮し、不要なファイルやディレクトリを除外します。
依存パッケージ、ビルド成果物、キャッシュなどを除いた一覧を確認したいときに便利です。

## 新しいファイルを見つける

```bash
richls --new-mark .
```

`--new-mark` を指定すると、24時間以内に作成されたファイルに `"new"` を付与して表示します。
現行実装では、詳細表示上で新しいファイルの印が表示されます。

## 並び替える

```bash
richls --sort name .
richls --sort size .
richls --sort mtime .
```

`--sort <key>` では、表示順を変更できます。
指定できる値は `name`, `size`, `mtime` です。

## よく使う組み合わせ

サイズの大きいファイルを確認します。

```bash
richls -l --humanize --sort size .
```

プロジェクトの構成を README の概要付きで確認します。

```bash
richls --tagline --respect-ignore .
```

最近追加または更新された項目を確認します。

```bash
richls --new-mark --sort mtime .
```

PDF が多いディレクトリで、資料名を確認します。

```bash
richls --pdf-title --sort name ./docs
```
