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
        // <- val はまだ定義されていない為使えない
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

// 参照と借用について...

pub fn ref_and_bor() {
  // 参照と借用
    // 所有権を奪わない代わりに引数で値を参照してみます
    let hii = String::from("Hiiiii!");
    let hii_len = cal_len(&hii); // cal_len関数は 変数hiiを参照する。故にhii変数は参照されただけなので生存
    println!("`{}`の長さは {}", hii, hii_len); // 変数hii, hii_len 共に使用可能
    // このように関数の引数に参照を取ることを借用と呼ぶ
    // 借用した値は変数と同じくデフォルトでは不変
}

pub fn mutable_ref() {
   // 可変な参照
    // 可変にするためには
    // まず参照する変数が可変出なければならない
    let mut mut_hii = String::from("Hiiiii!");
    // そして、&mut で可変参照を作る
    change(&mut mut_hii);

    // しかし、1つのデータの可変な参照は特定のスコープで1つしか作れない
    let mut soime = String::from("Hatefu");
    let soime_a = &mut soime;
    //let soime_b = &mut soime; これはだめ、エラー

    // 上記の仕様の利点としてコンパイルのにデータ競合を防ぐことができること
    // データ競合とは以下のようなことをいう
    //   2つ以上のポインタが同じデータに同時にアクセスする。
    //   少なくとも1つのポインタがデータに書き込みを行っている。
    //   データへのアクセスを同期する機構が使用されていない。

    // とはいえその可変参照が消滅すれば再度作ることが可能
    let mut sheto = String::from("Etchev");
    {
        let kode1 = &mut sheto;
    } // ここで変数kode1 がスコープを抜ける為、変数sheto の新しい可変参照を作ることが出来る
    let kode2 = &mut sheto;

    // なお、不変参照をしている間は可変参照をすることはできない
    let mut nyatod = String::from("Shydampa");
    let tebt1 = &nyatod;
    let tebt2 = &nyatod;
    //let tebt3 = &mut nyatod; これはだめ、エラー
}

   // 宙に浮いた参照
    /*

    fn gomi() -> &String {
    let s = String::from("Helloo"); // 新しい String型
    &s // s を返す...
    } // ところがここで s を drop()してしまい、&Stringが存在しない値を参照しようとしてしまい危険

    // 下の館数ならおｋ

    fn kami() -> String {
    let s = String::from("Helloo");
        s // ここで所有権がムーブされる
    } 
    
    */

pub fn slice() {
   // スライス型
    // スライスは所有権のないデータ型

    // それはさておき as_bytesメソッドで String型をバイト配列(ここでは u8型の配列)にしてみる
    let abab = String::from("AB ab");
    println!("{:?}", abab.as_bytes()); //< [65, 66, 97, 98]

    // それはさておき以下の関数シグネチャを見てみます
    // ※シグネチャ = 宣言  (シグニチャともいう)
    fn first_word(arg:&String) -> usize {
        let bytes = arg.as_bytes(); // arg をバイト配列に:&[u8]
        // iterメソッドで bytes配列の要素を返し、enumerateメソッドでそのまま返す
        for (i, &bytes_elm) in bytes.iter().enumerate() { // i はなんかいるもの
            // byutes_elm に `b' '` が来たら i を返す
            // `b' '` は半角空白のバイトリテラル
            // つまり bytes の半角スペースがある位置(インデックス)を返す
            if bytes_elm == b' ' { return i; }
        }
        // 半角空白が見つからなかったら arg の長さを返す
        arg.len()
    }
    let word = first_word(&abab); // "AB ab" を入れてみる
    println!("{}", word); //< 2
    // ただしこれでは abab が改変された際に word が同期されず、ただの役立たずと化す
}

pub fn string_slice() {
   // 文字列スライス
    // String の一部への参照のこと
    //``` &val[start_index..end_index]
    // start_index はスライスの開始地点(index)で、end_index は最終地点(index)+1
    // そして start_index や end_index は書かなかった場合それぞれ String の先頭, 末尾になる
    // `..` のみの場合 String全体となる
    let kelpa = String::from("SU KONBU");
    let su = &kelpa[..2]; // SU
    let konbu = &kelpa[3..]; // KONBU 
    let u_kon = &kelpa[1..6]; // U KON
    println!("{}\n{}\n{}", su, konbu, u_kon); //< SU\nKONBU\nU KON

    // これを用いて先ほどの first_word関数を書き直します
    fn first_word2(arg:&String) -> &str {
        let bytes = arg.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            // --ここまでは一緒
            if item == b' ' { return &arg[0..i]; } // if が通ったら arg の先頭から空白までの文字列スライス(つまり arg の初めの単語)を返す
        }
        // 空白がなかったら全体を返す
        // なお普通に arg を返すと String型になってしまうため
        // 文字列スライスで全体を &str として抜き取る
        &arg[..]
    }
    let mut pnf = String::from("Page not found:("); // あとであることを解説するので可変にする
    let word2 = first_word2(&pnf); // pnf の 初めの単語となる "Page" を word2 に代入
    // 何かへ不変参照がある時、さらに可変参照を得ることはできない故以下のようなことをするとエラーが出る
    //pnf.push_str("\n404");
    println!("{}", word2); //< Page
}

   // 文字列リテラルはスライスである
    /*let strrr = "MOJI!!!"; */
    // ここでの strrr は &str型
    // バイナリのその特定の位置を指すスライス
    // そしてこれが文字列が不変である理由にもなっている。要するに &strは不変な参照である

pub fn stringslice_in_arg() {
  // 引数での文字列スライス
   // しかし first_word2関数のままでは引数に &str型を入れることができない

   // 先程まで関数シグネチャで引数の型を String と指定していたが、
   // &str にすれば String型, &str型 両方を受け付けれる
   // そして文字列スライスされた値は &str ...
   // つまり &str にすることによって first_word2関数の引数に文字列スライスを使えるようにできる
   fn first_word3(arg:&str) -> &str { let bytes = arg.as_bytes();
       for (i, &item) in bytes.iter().enumerate() { if item == b' ' {return &arg[0..i];} }
       &arg[..]
   }
   println!("{}", first_word3("HAKATA NO SHIO")); //< HAKATA
}

pub fn other_slice() {
   // 他のスライス
    // 文字以外も一部の参照が可能
    let kotue = [1, 2, 3, 4, 5];
    let sackew = &kotue[2..4]; // [3, 4] :&[i32]
    println!("{:?}", sackew);
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

fn cal_len(arg:&String) -> usize { 
    arg.len() 
} // ここで arg はスコープから抜けるが参照しているだなため drop関数も呼ばない

//            vvv 関数定義側でも mut はつける
fn change(arg:&mut String) {
    arg.push_str(" Wooold!!");
    println!("{}", arg);
}