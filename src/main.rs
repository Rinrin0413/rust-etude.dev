//https://doc.rust-jp.rs/rust-by-example-ja/hello.html

// これはコメントアウト。
/* コンパイラ時に無視される */

mod lib; // data_types.jsonを関連付けてる

// main関数
// コンパイルされたバイナリが実行されるとこの関数が呼び出されます
fn main() 
{
    // 変数定義 "let <変数名> = <代入物>;"
    // Rust では変数はデフォルトで不変
    let test_val = "変数の中身";
    // 引数に投げれば出力可能
    println!("{}", test_val);

    // let の後に mut を入れれば可変変数にできる
    let mut mutable_val = "可変変数だお";
    println!("{}", mutable_val);
    mutable_val = "可変変数だネ"; // 書き換え
    println!("{}", mutable_val);

    // 別に定数もある。定数名は必ず大文字である、そして注釈が必要
    const TEST_TEISU:u32 = 64_000; // 64,000
    println!("{}", TEST_TEISU);

    // let を再度使って元の変数と同じ名前で変数を定義できる
    // これを覆い隠しという
    // 可変変数とは違い定義(覆隠)後、不変変数とできる
    let shad = 16;
    println!("{}", shad); // 16
    let shad = shad*2; // 2倍
    println!("{}", shad); // 32
    let shad = shad*2; //2倍
    println!("{}", shad); // 64
    // 可変変数でも型は変えられないが、覆い隠しでは型も変えられる
    let spaces = "   "; // こいつは文字列
    let spaces = spaces.len(); // こいつは数値になる
    println!("{}", spaces);

    let atai = "128";
    println!("{}", atai);
    // str であるatai変数を u32になるように覆い隠す
    let atai:u32 = atai.parse().unwrap();
    println!("{}", atai*2); // 2倍にしてみる

   //✔データ型のソースコードは lib/data-types.rs を参照
    lib::data_types::scala();
    
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
