//https://doc.rust-jp.rs/rust-by-example-ja/hello.html

// これはコメントアウト。
/* コンパイラ時に無視される */

// main関数
// コンパイルされたバイナリが実行されるとこの関数が呼び出されます
fn main() 
{
    // pythonの f"{}" みたいなやつ。
    // ただし引数に代入するものを置く
    println!("1 {} 3 {}", 2, 4);


    // 引数のインデックスを入力して並替可能
    println!("A{1}C{0}", "D", "B");

    // 名前指定も可能
    println!(
        "あ{i}う{u}え{o}お",
        i="い",
        u="う",
        o="お"
    );

    // : の後ろにフォーマット型を指定できる
    let int = 14;
    println!("bin(2進数) : {x:b}" ,x=int);
    println!("oct(8進数) : {x:o}" ,x=int);
    println!("hex(16進数) : {x:x}" ,x=int);
    println!("exp(指数表記) : {x:e}" ,x=int);
    println!("HEX(大文字16進数) : {x:X}" ,x=int);
    println!("EXP(大文字指数表記) : {x:E}" ,x=int);

    
    // x:>n でn個の半角空白の先に x が置かれる
    // x:<n だと x の後に半角空白がn個置かれる
    // n に変数(?)を使いたい場合は n$ にする
    println!("( 'ω'){kao:>dist$}ｿｰｼｬﾙﾃﾞｨｽﾀﾝｽ {dist}m", kao="('ω' )", dist=10);
    // x:^n でxの左右にそれぞれ n/2 個の半角空白が置かれる
    println!("( 'ω'){kao:^dist$}ｿｰｼｬﾙﾃﾞｨｽﾀﾝｽ 5m", kao="('ω' )", dist=10);
    // [なんか動かない] 空白の代わりに0を使うことも可能
    // [なんか動かない] println!("( 'ω'){kao: >0dist$}ｿｰｼｬﾙﾃﾞｨｽﾀﾝｽ {dist}m", kao="('ω' )", dist=10);
    
    // i32 を保持する Structure という名の構造体を定義
    #[derive(Debug)]//#[allow(dead_code)]
    struct Structure(i32);
    
    // このようにカスタム型を用いる場合、少し扱いが複雑になる。
    // :? だとそのまま文字列として出力(?)
    // :#? で見やすくする
    println!("This struct `{:#?}` won't print...", Structure(3));
}
