// https://doc.rust-jp.rs/rust-by-example-ja/hello.html
// https://doc.rust-jp.rs/book-ja/

// 2連スラッシュを打つととそこからその行の後ろはコメントアウトと見なされる。
/* コンパイラに無視されます */

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

   //✔データ型のソースコードは ./lib/data_types.rs を参照
    lib::data_types::scala(); // スカラ型
    lib::data_types::complex(); // 複合型
    
   // println!関数特集
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

   // 関数定義と引数
    // 関数の定義は  fn <関数名>([引数]) { <処理> }  で出来る
    // 引数は必ず型を指定しなければならない
    // 複数の引数を持たせる場合はカンマで区切ることで可能
    // この例では main関数の外で関数を定義している
    another_function(64, 1024);

   // 式と文
    // 文は、なんらかの動作をして値を返さない命令
    // 式は、結果値に評価されるもの

    // let で変数を定義する部分は文(let文)
    let hensu = "x";
    // println!関数で hensu変数を呼び出す部分は式
    println!("{}", hensu);
    // そして関数の定義する部分は文となる

    // 文は値を返さないので、let文を変数に代入することは出来ない
    // 例:  let x = (let y = 6);

    // スコープを作る波括弧 {} は式
    // 故に let y = { 8 + 5 }  のようなことをしても返せる
    let modo = 5;
    let weyi = { let modo = 3; modo + 1 };
    println!("y変数の中身 : {}", weyi); // 4
    println!("元のスコープでのx : {}", modo); // 5
    // ここで modo + 1 にセミコロンを付けてしまった場合
    // 式である modo + 1 が文になってしまう

   // 戻り値
    // 関数は呼び出してきたコードに値を返すことができる
    // fn <関数名>([引数]) -> <戻り値の型> { <処理>; 戻り値 }
    let fyv = return_five(); // let fyv = 5; をしているのに等しい
    println!("return_five関数の戻り値は {}", fyv); // 5

    // もちろん引数に値を入れて演算の結果を返すことも可能
    println!("{}", plus_one(16)); 

   //✔フロー制御(条件分岐やループ)のソースコードは ./lib/flow_control.rs を参照
    lib::flow_control::bi_if(); // if
    lib::flow_control::lp_loop(); // loop
    lib::flow_control::lp_while(); // while
    lib::flow_control::lp_for(); // for

   //✔変換のスコープやString型, 所有権についての説明, ソースコードは ./lib/ownership.rs を参照
    lib::ownership::val_and_scope(); // 変数とスコープ
    lib::ownership::type_string(); // String型
    lib::ownership::interact_method(); // 変数とデータの相互作用法
    lib::ownership::fun_and_ownership(); //所有権と関数
    lib::ownership::return_and_scope(); // 戻り値とスコープ

   //✔参照や借用についての説明, ソースコードは同じく ./lib/ownership.rs を参照
    lib::ownership::ref_and_bor(); // 参照と借用
    lib::ownership::mutable_ref(); // 可変な参照

   //✔スライス型について説明、ソースコードも同じく ./lib/ownership.rs を参照
    lib::ownership::slice(); // スライス型
    lib::ownership::string_slice(); // 文字列スライス
    lib::ownership::stringslice_in_arg(); // 引数での文字列スライス
    lib::ownership::other_slice(); // その他のスライス

   //✔構造体についての説明、ソースコードは ./lib/structure.rs を参照
    lib::structure::structure(); // 構造体とインスタンス化
    lib::structure::ex_refactoring(); // 構造化を用いたリファクタリング
    lib::structure::method(); // メソッド
    lib::structure::relate_fn(); // 関連関数

   //✔Enum とパターンマッチングについての説明、ソースコードは ./lib/enumerate.rs を参照
    lib::enumerate::enumerate(); // 列挙型について




    // i32 を保持する Structure という名の構造体を定義
    #[derive(Debug)]//#[allow(dead_code)]
    struct Structure(i32);
    
    // このようにカスタム型を用いる場合、少し扱いが複雑になる。
    // :? だとそのまま文字列として出力(?)
    // :#? で見やすくする
    println!("This struct `{:#?}` won't print...", Structure(3));
}


fn another_function(x:i32, y:i32) {
    println!("第1引数 : {}", x);
    println!("第2引数 : {}", y);
    println!("全引数の和 : {}", x + y);
}

fn return_five() -> i32 {
    5 // 戻り値
}

fn plus_one(x:i32) -> i32 {
    let wa = x + 1;
    wa // 戻り値( x + 1  を直接置くことも出来るが、変数も置けることを証明する為にあえて変数化)
}