# AIと遊ぶ
https://az.recazbowl.net/deft_web/

## 難易度について
- level 0
  ランダム打ち
- level 1
  1手読み（弱い）
- level 2
  2手読み （油断してると負ける）
- level 3
  4手読み、終盤8手完全読み（そこそこ強い）
- level 4
  8手読み、終盤16手完全読み（強い）
- level 5
  10手読み、終盤22手完全読み（重いので注意）

# Note


ビルド方法

```
git clone https://github.com/ikepggthb/deft_web.git
cd deft_web
git clone https://github.com/ikepggthb/deft-reversi-engine.git

cargo install wasm-pack
wasm-pack build --target web
```
