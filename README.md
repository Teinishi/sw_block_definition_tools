# Stormworks Block Definition Tools

Stormworks のパーツ定義 XML ファイルを見るためのツールです。パーツごとの各種属性値の確認、音声のプレビュー、3D 表示が可能です。また、属性を選択して全パーツのその属性の値の一覧を表示したり、値ごとにグループ化して表示したりすることができます。

## 使い方

起動したら、Open rom folder ボタンから `stormworks.exe` と同じフォルダにある `rom` フォルダを選択してください。`rom/data/definitions` 以下のパーツ定義 XML ファイルを一覧表示します。

画面上部のタブから Save image を選択すると、ブロックの画像を保存することができます。

## ビルド方法

[Rust のインストール](https://doc.rust-jp.rs/book-ja/ch01-01-installation.html) が必要です。

次の手順でこのリポジトリをクローンし、ビルドすることができます。

```
> git clone https://github.com/Teinishi/sw_block_definition_tools.git
> cd sw_block_definition_tools
> cargo build --release
```

ビルドした実行可能ファイルは `target/release/sw_block_definition_tools.exe` に作成されます。
