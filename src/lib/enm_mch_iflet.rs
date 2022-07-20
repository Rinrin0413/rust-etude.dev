#![allow(unused)] // 未使用変数とかの許可定期
pub fn enumerate() {
// DOC.6-1
  // Enum を定義する
    // Enum とは列挙型の別名です
    // 取る可能性のある全ての値を並べあげ、1つ1つ数えあげてくれます
    // 例えば ipアドレスは v4 と v6 があります
    // 必ずどちらかになりますが両方にはなることはありません
    // しかし別の型として扱うわけにもいきません

    // IpAddrKind という列挙型を定義して ipアドレスがなりうる種類である v4 と v6 を列挙することで 表現できます。
    enum IpAddrKind {
        V4,
        V6,
    }
    // これで IpAddrKind型という独自の型として使えます
    // この V4 や V6 は列挙子といいます

   // Enum の値
    // 列挙子のインスタンスの呼び出し
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // 2連コロンを使って区別されていますが同じ型です

    // こうすると管理がしやすいです
    enum IpAddr {
        V4(String),
        V6(String),
    }

    // レかも！！
    // 列挙子それぞれで型が異なっていても良い(素晴らレい)
    enum IpAddrII { V4(u8, u8, u8, u8), V6(String), }
    let home = IpAddrII::V4(127, 0, 0, 1);
    let loopback = IpAddrII::V6(String::from("::1"));

    // との樣に ipアドレス用の列挙型を作りましたが...
    // 元々標準ライブラリに ipアドレス用の列挙型があるので贵樣の出番てはない
    // リンク : https://doc.rust-lang.org/std/net/enum.IpAddr.html

    // ということで別の列挙型を作りましょう
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // Quit : データ無し
    // Move : 匿名構造体を含む
    // Write : String を含む。
    // ChangeColor : 3つの i32値を持つタプルを含む。

   // Option enumとNull値に勝る利点
    // Rust には値がないことを意味する null(トイレットペーパーホルダのみの状態)が存在しません
    // しかし値の存在不在が判断できる Option<T> という列挙型が標準ライブラリにあります
    // 以下のように定義されています
    /*
    enum Option<T> {
        Some(T),
        None,
    }
    */
    // <T> という記法はジェネリック型引数といい今後学びます
    // 軽く説明すると、Option列挙型 の Some列挙子 が型のデータを1つだけ持つことができることを意味しています

    // なお、有能なので明示的にスコープに導入する必要は(ないです)
    // Some() も None も Option:: 無しで扱えます
    // 以下は使用例
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number:Option<i32> = None; // こいつが null の代役
    // なお、None値だけ投げられても型が分からずコンパイラが混乱するのでジェネリック型引数に値を入れてあげる

    // Option<T> と T(ここではどんな型でも良) は異なる型なのでコンパイラがOption<T>値を確実に有効な値かのようには使用させてくれない
    let x:i8 = 5; //:i8
    let y:Option<i8> = Some(5); //:Option<i8>
    // なので下のコードはエラー
    //let sum = x + y;

    // こうすれば良い
    let sum = x + match y { // match式で y が取りうる値から条件分岐
        Some(i8) => y.unwrap(), // Some(i8) のパターンなら T を
        _ => 0, // アンダーバー(それ以外の意)なら適当に0でも返す(いけません)
    };
    //細かいことは次の章(6-2)で解説する
}

pub fn match_fc() {
// DOC.6-2
  // matchフロー制御演算子
    // パターンによって条件分岐してくれるのが match です
    // 札の種類で分けるやつを作ってみる
    enum Bill {
        SenYen,
        GosenYen,
        IchimanYen,
    }
    fn satsu_wake(bill:Bill) -> u32 {
        match bill { // bill の返す値が以下のアームに合致するかによって処理を分岐する
            Bill::SenYen => 1000, // 型が Bill で 列挙子が SenYen なら 1000 を
            Bill::GosenYen => 5000, // 型が Bill で 列挙子が GosenYen なら 5000 を
            Bill::IchimanYen => 10_000, // 型が Bill で 列挙子が IchimanYen なら 10,000 を
            _ => { println!("それは↓札じゃねぇ↑( ﾟДﾟ)ﾉ⌒⑤"); 0 } // それ以外なら勝ちゼロって投げつける(アンダーバーはそれ以外という意)
                                                               // この中括弧については真下で開設する
        }
    }

    // アーム内で複数の処理を行いたい場合は新スコープ展開してやる
    match 16i8 {
        i8 => {
            println!("贵樣ばi8た！");
            println!("正レい.");
        }
        _ => println!("んなわけが..."),
    }

   // 値が束縛されるパターン
    // まずデータを持つ列挙子を入れてみましょう
    #[derive(Debug)]
    enum Nioi { Kusai, Kusakunai }
    enum BillII {
        SenYen,
        GosenYen,
        IchimanYen,
        NisenYen(Nioi),
    }
    fn nisenyen_kusaikana(bill:BillII) {
        match bill {
            BillII::NisenYen(status) => println!("この二千札は {:?}", status),
            _ => println!("二千札だけよこしな"),
        }
    }
    // アーム内にある status に列挙子が保持するデータを束縛して使えます
    // 例えば以下の場合...
    nisenyen_kusaikana(BillII::NisenYen(Nioi::Kusai)); //< この二千札は Kusai
    // status には Nioi::Kusai が入る

   // Option<T>とのマッチ
    // Option<T> でも同じように♪
    fn plus_one(x:Option<i32>) -> Option<i32> {
        match x {
            None => None, // 中に値がなければ関数はNone値を返す
            Some(i) => Some(i + 1), // i に Some が持つ値に束縛されて、そのまま計算してまた Someとして返される
        }
    }
    let five = Some(5);
    let six = plus_one(five); //< 6
    let none = plus_one(None);//< None

   // マッチは包括的
    // None の処理を忘れるとコンパイラがキレます

   // _というプレースホルダー
    // さっきからたまに使っていますが _ は else　みたいに動いてくれます
    let some_u8_value:u8 = 0;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 1, 3, 5, 7 のいずれでもない場合ユニット型を返す(つまり何もしない)
    }
    // しっかしながいてずれ...
    // そラ思た贵樣には！
    // DOC.6-3 参照
}

pub fn if_let() {
// DOC.6-3
   // if letで簡潔なフロー制御
    // if let記法で if と let を短くマッチさせられます
    // 例えば Some(3) がマッチしたときに処理する分岐を match で書くと
    let some_u8_value_ii = Some(0u8);
    match some_u8_value_ii { Some(3) => println!("three"), _ => (), }
    // 書かないといけない定型コードが多すぎます

    // if let記法を使ってみると
    if let Some(3) = some_u8_value_ii { println!("three"); }
    // 短いです

    // else も使えます match でいう _プレースホルダです
    if let Some(5) = some_u8_value_ii {
        println!("five"); 
    } else {
        println!("not five");
    }
}