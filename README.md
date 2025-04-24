# mirudi

GitHub の diff を簡単にする CLI ツール

## ビルド方法

```bash
cargo build --release
```

## インストール方法（初回のみ）

### cargo がある人向け

```bash
git clone git@github.com:minty1202/mirudi.git
cd mirudi
make install
export PATH="$HOME/.local/bin:$PATH"
```

### cargo がない人向け

```bash
curl -L https://github.com/minty1202/mirudi/releases/latest/download/mirudi -o ~/.local/bin/mirudi
chmod +x ~/.local/bin/mirudi
export PATH="$HOME/.local/bin:$PATH"
```

## アンインストール方法（誰でもできるよ）

```bash
rm -f ~/.local/bin/mirudi
```

## リリース方法 GitHub Releases

Cargo.toml の version を元にリリースされます

```bash
bash release.sh "リリースノート"
```
