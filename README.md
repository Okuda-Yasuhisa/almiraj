# Almiraj

## 概要
Almirajは、Rustの安全性と所有権のシステムをGPUプログラミングに適用する新しい言語です。
これにより、開発者はunsafeコードを使用せずに安全にGPU向けのコードを記述できます。
> [!Important]
> AlmirajはRustの影響を強く受けている言語です。  
> Almirajの習得にRustは必須ではありませんが、所有権などのシステムの大部分はRustのシステムと同じです。  
> Almirajの開発時にRustのスキルがあると、Almirajの開発が容易になるでしょう。

## 言語仕様
- **拡張子**: すべてのソースファイルは `.almiraj` という拡張子を使用します。
- **静的型付け**: 変数、関数パラメータ、戻り値は明示的な型指定が必要です。  
  例: `uint8 x = 10;`
- **関数定義**:  
  関数は以下のように定義します。  
  例:
  ```
  global fn main(uint8 n) {
      // 処理
  }
  ```

## 型システム
Almirajでは、符号無しの型は先頭に `u` を付けます。  
たとえば、`uint` は unsigned int を表し、その後ろに使用するメモリのバイト数を指定します。    

### プリミティブ型
Almirajは以下のプリミティブ型を提供します：

#### 符号なし整数型
- `uint8`: 8ビット符号なし整数 (0 to 255)
- `uint16`: 16ビット符号なし整数 (0 to 65,535)
- `uint32`: 32ビット符号なし整数 (0 to 4,294,967,295)
- `uint64`: 64ビット符号なし整数 (0 to 18,446,744,073,709,551,615)

#### 符号付き整数型
- `int8`: 8ビット符号付き整数 (-128 to 127)
- `int16`: 16ビット符号付き整数 (-32,768 to 32,767)
- `int32`: 32ビット符号付き整数 (-2,147,483,648 to 2,147,483,647)
- `int64`: 64ビット符号付き整数 (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)

#### 符号なし浮動小数型
- `ufloat8`: 8ビット符号なし浮動小数
- `ufloat16`: 16ビット符号なし浮動小数
- `ufloat32`: 32ビット符号なし浮動小数
- `ufloat64`: 64ビット符号なし浮動小数

#### 符号付き浮動小数型
- `float8`: 8ビット符号付き浮動小数
- `float16`: 16ビット符号付き浮動小数
- `float32`: 32ビット符号付き浮動小数
- `float64`: 64ビット符号付き浮動小数

#### 真偽型
真偽型は`bool `と宣言する必要があります。真の場合は`true`、偽の場合は`false`を代入します。

### 量子化向けの特殊整数型
Almirajは量子化計算向けに、1バイト未満の小さなメモリサイズもサポートします：

- `uint1`: 1ビット符号なし整数 (0 to 1) - 2値量子化用
- `uint2`: 2ビット符号なし整数 (0 to 3) - 4値量子化用
- `uint4`: 4ビット符号なし整数 (0 to 15) - 16値量子化用

これらの型は主に以下の用途で使用されます：
- ニューラルネットワークの量子化
- メモリ効率の最適化
- 高速な演算処理

注意：これらの型は内部的には1バイトにアライメントされますが、
演算時には指定されたビット数でマスクされます。

### 型名の命名規則
- 符号なし整数型は `u` プレフィックスを使用 (例: `uint8`)
- 符号付き整数型は直接数値を指定 (例: `int8`)
- 数値は使用するビット数を示します

## 文法仕様

### 変数宣言
変数宣言は必ず型を明示的に指定する必要があります：
```almiraj
uint8 counter = 0;  // 使用するメモリは8バイト
bool flag = true;   // 使用するメモリは1バイト(他の変数と一緒にして1バイトと扱います)
```

### 関数定義
関数定義も戻り値の型を明示的に指定します：
```almiraj
fn add(uint32 a, uint32 b) -> uint32 {
    return a + b;
}

global fn kernel(uint32 n) {
    uint32 idx = blockIdx.x * blockDim.x + threadIdx.x;
}
```

### 制御構文
条件式の型は必ずbool型である必要があります：
```almiraj
if (x > 0) {
    // 処理
}

while (is_running) {
    // 処理
}

for (uint8 i = 0; i < 10; i = i + 1) {
    // 処理
}
```

### 型推論
現バージョンではすべての型を明示的に指定する必要があります。
型推論は将来のアップデートで実装予定です。
