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