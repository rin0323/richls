---
title: "richls Install"
date: 2026-06-26T00:00:00+09:00
draft: false
summary: "richls をローカル環境へインストールするための想定手順です。"
description: "Cargo を使った richls のビルド、インストール、動作確認の方法を説明します。"
tags: ["richls", "install", "cargo"]
categories: ["Guide"]
ShowToc: true
TocOpen: true
ShowReadingTime: true
ShowBreadCrumbs: true
ShowPostNavLinks: true
---

このページでは、`richls` をローカル環境で使うためのインストール手順を説明します。
現時点では、ソースコードから Cargo でビルドして導入する方法を基本とします。

## 前提

`richls` は Rust 製の CLI ツールとして利用する想定です。
事前に Rust と Cargo をインストールしてください。

```bash
rustc --version
cargo --version
```

これらのコマンドでバージョンが表示されれば準備完了です。

## ソースコードからインストールする

リポジトリを取得します。

```bash
git clone https://github.com/rin0323/richls.git
cd richls
```

ローカルのソースからインストールします。

```bash
cargo install --path .
```

インストール後、次のコマンドで実行できることを確認します。

```bash
richls --version
richls --help
```

## ビルドだけ行う

インストールせずにビルド成果物を確認したい場合は、release ビルドを実行します。

```bash
cargo build --release
```

ビルド後の実行ファイルは通常、次の場所に作成されます。

```bash
./target/release/richls
```

直接実行する場合は、次のように呼び出します。

```bash
./target/release/richls --help
```

## crates.io 公開後のインストール

将来 crates.io に公開した場合は、次の形式でインストールできます。

```bash
cargo install richls
```

この方法を案内する場合は、公開済みのパッケージ名が `richls` であることを確認してください。

## PATH を確認する

`cargo install` で入れたコマンドは、通常 `~/.cargo/bin` に配置されます。
`richls` が見つからない場合は、PATH に `~/.cargo/bin` が含まれているか確認してください。

```bash
echo $PATH
```

必要に応じて、利用しているシェルの設定ファイルに次のような設定を追加します。

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## 動作確認

インストールできたら、任意のディレクトリで一覧表示を試します。

```bash
richls
richls -l .
richls --respect-ignore --sort name .
```

詳細表示や追加情報の表示は、用途に応じてオプションを組み合わせて使います。
