# AIと遊ぶ
https://az.recazbowl.net/deft_web/

## 難易度について
- level 0
  ランダム打ち
- level 1 (初級)
  1手読み
- level 2 (中級)
  2手読み
- level 3 (上級)
  4手読み & 終盤8手完全読み
- level 4
  8手読み & 終盤16手完全読み
- level 5
  10手読み & 終盤22手必勝読み & 終盤20手完全読み
- level 6
  12手読み & 終盤24手完全読み & 終盤22手完全読み

# Note


ビルド方法

- Rustと、wasm-packが必要です。

```
git clone https://github.com/ikepggthb/deft_web.git
cd deft_web
git clone https://github.com/ikepggthb/deft-reversi-engine.git

cargo install wasm-pack
wasm-pack build --target web
```
