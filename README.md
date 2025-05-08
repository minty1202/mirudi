# mirudi

GitHub の diff を簡単にする CLI ツール

## インストール方法（初回のみ）

### cargo がある人向け

```bash
git clone git@github.com:minty1202/mirudi.git
cd mirudi
make install

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
fi
```

### cargo がない人向け

```bash
mkdir -p ~/.local/bin
curl -fL https://github.com/minty1202/mirudi/releases/latest/download/mirudi -o ~/.local/bin/mirudi
chmod +x ~/.local/bin/mirudi

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
fi
```

## アンインストール方法（誰でもできるよ）

```bash
rm -f ~/.local/bin/mirudi
```

## セットアップ

下記を実行して、ベースとなるブランチを設定してください。

```bash
mirudi init

ex) mirudi init develop
```

## 使い方

### Web

web を実行すると、ブラウザが立ち上がります。
その状態で対象の行を選択すると、差分が表示されます。

```bash
mirudi web
```

### FF

ff を実行すると、ターミナル上で差分が表示されます。

```bash
mirudi ff ベースブランチ側のファイル行 対象ブランチ側のファイル行

ex) mirudi ff 1-10 1-20
ex) mirudi ff 1-10 1-20 -c -p src/main.rs
```

#### 主なオプション

- `-h` / `--help` : ヘルプを表示
- `-b` / `--branch` : 対象のブランチを指定
- `-c` / `--current` : 対象のブランチを現在のブランチに指定
- `-o` / `--old-path` : ベースブランチ側のファイルを指定
- `-n` / `--new-path` : 対象ブランチ側のファイルを指定
- `-p` / `--path` : -o / --old-path と -n / --new-path の両方を指定
- `-m` / `--mode` : diff のモードを指定
  - `slice` : 標準モード
  - `words` : 単語モード
  - `lines` : 行モード
  - `chars` : 文字モード
