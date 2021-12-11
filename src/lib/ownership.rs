#![allow(unused)] // 変数とか使わなくても怒られないやつ
// DOC.4-1
 // 所有権とは
    // 所有権はメモリを安全に扱うための Rust の機能

   // スタックとヒープ
    // 所有権を学ぶ上でスタックは必要な知識

    // スタックは使用できるメモリの1つ
    // 新しく値を得ると古い値は排除される故高速に処理される

    // ヒープも使用できるメモリの1つ
    // データ(値など)を置くときにスペースを求め、OS が空の領域を見つけてポインタ(アドレス)を返す

   // 所有権規則
    // 所有権を使う中でこれらのルールに応じなさい
    // 1.Rustの各値は、所有者と呼ばれる変数と対応している。
    // 2.いかなる時も所有者は一つである。
    // 3.所有者がスコープから外れたら、値は破棄される。

pub fn val_and_scope() {
   // 変数スコープ
    // 疑似的なスコープ( {} )を作って説明します
    {
        // <- s はまだ定義されていない為使えない
        let val = "あたい"; // val が定義されたので使用可能に
        println!("{}", val); // val で作業をする...
    } // このスコープは終わった為 val は使えない
    // つまり、val がスコープに入ると有効に、スコープを抜けるまで有効なままである
}

pub fn type_string() {
   // String型
    // doc.3-2で学んだ型は全てスタックに保管され、スコープが終わるとスタックから排除される
    // ヒープに確保されるデータ型を観察し、コンパイラがどう それを排除すべきタイミングを求めているかを見ていく
    // ここではヒープに保管される型の例として String型を使用する

    // &str型は扱いやすいが不変。しかし String型は可変故ユーザ入力に応じた文字列の変更も可能
    // この型はヒープにメモリを確保する為、コンパイル時にサイズ不明なテキストも保持することが可能

    // from関数で文字列リテラルから始まる String型を生成できる
    let mut string_hello = String::from("Hello");
    // この二重コロンは String型直下の from関数(Stringの関連関数)を特定する演算子

    // String型の変数に push_str関数でリテラルを付け加える事が可能
    string_hello.push_str(", World!");
    println!("{}", string_hello); //< Hello, World!

    // String の関連関数new で空の String型 を生成できる
    let string_new = String::new();
    println!("{}", string_new); //< 
}

   // メモリと確保
    // &str型は中身がコンパイル時に判明しているため、文字列が直接出力される
    // なので &str型は高速で効率的になるがしかしこの型ではコンパイル時にサイズ不明という状況は作れない
    // String型は可変かつ伸長可能で、コンパイル時にサイズ不明な状況でヒープに確保することが可能

pub fn interact_method() {
  // 変数とデータの相互作用法
  
   // 1.ムーブ
    // String型は以下のようなデータで構成されている
    // { "ptr":["H","e","l","l","o"], "len":5, capacity:5 }
    // ptr はヒープに保管され、len は文字列の大きさ(byte)、capacityは許容量
    // もし String型の変数a を変数b にそのまま代入した場合、変数b は変数a と同じ場所にある ptr を参照する
    // そして変数b　の出現により変数aは排除される。このことを「a は b にムーブされた」という
    let string1_mv = String::from("Hello");
    let string2_mv = string1_mv;
    // ここで string1 はもう使えない

   // 2.クローン
    // もし仮に万が一どうしてもヒープデータ(String型でいう ptr)の複製が必要ならば cloneメソッドを使います
    let string1_cl = String::from("Hi, there");
    let string2_cl = string1_cl.clone(); // ヒープデータをコピー
    println!("{}\n{}", string1_cl, string2_cl); //< Hi, there\nHi, there
    // ヒープデータが2倍になる為、もし仮に万が一どうしても必要な時にのみクローンをする

   // 3.コピー
    // i32型は先ほど話した通りスタックに保管されるため以下エラーを出さない
    let x_cp = 8;
    let y_cp = x_cp;
    println!("x_cp = {}\ny_cp = {}", x_cp, y_cp);

    // Rustには Copyトレイトという特別な注釈があり、 スタックに保持される型に配置することができる
    // Copy の実装された型はスタックに保持される
    // 例としては以下の型が Copy の実装された型だ
    //    全整数型(int)
    //    論理値型(bool)
    //    全浮動小数点型(float)
    //    文字型(char)
    //    Copy の実装された型だけを持つタプル  例えば (i32, i32) は Copy だが (i32, String) は違う
}

pub fn fun_and_ownership() {
  // 所有権と関数
    // 関数に変数を入れることも代入と似た挙動となる

    let hasnt_copy = String::from("Oveve"); // hasnt_copy変数がスコープに出現
    takes_ownership(hasnt_copy); // hasnt_copy が takes_ownership関数にムーブされ、hasnt_copy変数は排除される
    // hasnt_copy変数は死んだので使えない
    let has_copy = 5; // has_copy変数がスコープに出現
    makes_copy(has_copy); // has_copy変数が makes_copy関数にムーブされるが、i32型は Copy 故そのままコピーされる
    // has_copy変数 は生存し続けた為ここでも使用可能

   // 戻り値とスコープ
    // 値を返す事でも所有権は移動する

    // 適当なスコープ
    {
        let saitou = String::from("たいせつな財産");
        let gokudow_kumichow = nusumu(saitou); // サイトウの財産が nusumu()によってムーブされその返り値が組長にムーブされる
    } // ここで gokudow_kumichowはスコープを抜けて財産が捨てられる( drop() )

    // 関数に入れた時に所有権を奪われないようにするには(正確には複製)

    //適当な スコープ
    {
        let kimura = String::from("岩倉具視入門書");
        let (kimura, kumichow) = copy(kimura); // キムラが本をコピーする(いけません)
                                               // 同時にコピーされた本が組長に返される
        println!("組長 has {}\nキムラ has {}", kumichow, kimura);
    }
}

pub fn return_and_scope() {
  // 戻り値とスコープ
    // 値を返す事でも所有権は移動する

    // 適当なスコープ
    {
        let saitou = String::from("たいせつな財産");
        let gokudow_kumichow = nusumu(saitou); // サイトウの財産が nusumu()によってムーブされその返り値が組長にムーブされる
    } // ここで gokudow_kumichowはスコープを抜けて財産が捨てられる( drop() )

    // 関数に入れた時に所有権を奪われないようにするには

    //適当な スコープ
    {
        let kimura = String::from("岩倉具視入門書");
        let (kimura, kumichow) = copy(kimura); // キムラが本をコピーする(いけません)
                                               // 同時にコピーされた本が組長に返される
        println!("組長 has {}\nキムラ has {}", kumichow, kimura);
    }
}

fn takes_ownership(arg: String) { // hasnt_copy変数がスコープに出現
    println!("{}", arg);
} // ここで hasnt_copy変数がスコープを抜け、drop関数(使われていたメモリを開放する)が呼ばれる。

fn makes_copy(arg: i32) { // has_copy変数がスコープに降臨
    println!("{}", arg);
} // has_copy変数 はスコープから抜ける。以上。それだけ。

// 財産を盗み手に入れる
//        vvvvvvvvvv     
fn nusumu(zaysan:String) -> String { 
    zaysan // 組長の手に渡る ^^^^^^^
}

fn copy(hon: String) -> (String, String) {
    let hon_copy = hon.clone(); // 本をコピー(はんざいです)
    (hon, hon_copy) // タプルにオリジナルの本とコピーの本を入れて戻り値で組長が手にとる
}