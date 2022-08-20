#![allow(unused)]

mod front_end {
    pub mod display {
        pub fn show_icon() {}

        fn show_header() {}
    }
    pub mod animation {
        fn rotate_icon() {}

        fn scale_up_header_in_hover() {}
    }
}

pub fn browse() { // browse関数定義
    // 絶対パス
    crate::front_end::display::show_icon(); // webdev/src/ がクレートルートとなる
    // 相対パス
    front_end::display::show_icon();
    
   // パスをpubキーワードで公開する
	// モジュールはコードの整理に役立つだけでなく プライバシー境界 も定義できる
	// Rust において全要素のプライバシーはデフォルトで非公開
	// 親モジュールの要素は子モジュールの非公開要素を使えないが子モジュールの要素はその祖先モジュールの要素を使える
	// pubキーワードを使うことで公開することができる
    // なお、front_endモジュールと browse関数は兄弟なので参照可能故 pubキーワードは不要

   // 相対パスをsuperで始める
    // 親モジュールから始まる相対パスは super を最初につけることで作れる
    // ファイルパス表記でいう `../` に似ている
}
    // super の使用例
fn oji() {}
mod oya {
    fn ko() { super::oji(); } 
}

   // 構造体とenumを公開する
    // 構造体と enum の公開にはいくつかの細かい設定がある
mod back_of_house {
    pub struct Breakfast { // メソッドの型枠に構造体を定義
        pub toast: String, // toastフィールドを公開に
        seasonal_fruit: String, // フルーツは非公開
    }
    impl Breakfast { // 構造体Breakfast の文脈で
        pub fn summer(toast:&str) -> Breakfast { // summer関数を公開で定義
            Breakfast { // 初期化省略記法で
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    // 夏にライ麦パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye"); // back_of_houseモジュールの Breakfastメソッドより summer関数でインスタンスを作成
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat"); // 構造体meal の toastフィールドの値を変更
    println!("I'd like {} toast please", meal.toast);
    // 食事についてくるフルーツを知ることも修正することも許されていないの以下はエラー
    //meal.seasonal_fruit = String::from("blueberries");
}

    // 一方で enum は公開するとその変数はすべて公開される
    // pub は enumキーワード の前にだけおけばよい
mod back_of_house_ii {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant_ii() {
    let order1 = back_of_house_ii::Appetizer::Soup;
    let order2 = back_of_house_ii::Appetizer::Salad;
}

    //✔以上で「パス指定やプライバシーの詳細についてI」の参照終了。以降再び ../../src/lib/packages_crates_modules.rs で path_and_privacy節後参照

mod front_of_house { // front_of_houseモジュールの定義
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::front_of_house::hosting;
// 上のおかげで front_of_house::hosting がこの改装で定義されているかのように使える
pub fn eat_at_restaurant_iii() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

    //  相対パスも使えます
mod front_of_house_ii { pub mod hosting_ii {pub fn add_to_waitlist_ii() {}} }
use self::front_of_house_ii::hosting_ii; // もちろん super でも良い
pub fn eat_at_restaurant_iv() {
    hosting_ii::add_to_waitlist_ii();
}

   // 慣例に従ったuseパスを作る
    // なぜ以下のようにしなかったか疑問に思うかもしれない
    //use crate::front_of_house::hosting::add_to_waitlist;
    // さすれば add_to_waitlist() だけで呼び出せるのに... と
    // もちろん動きますが
    // フルパスを書かずして、関数がローカルで定義されていないことが分かるようにするには
    // こちらの方が合理的

    // なお同じ名前を持つ親が異なる要素は二つ以上同じスコープに存在できません

   // 新しい名前をasキーワードで与える
    // 同じ名前の2つの要素をuseで同じスコープに持ち込むにはパスの後に as と新しいローカル名を指定するよ良い
    // 例で二種類の Result型の片方を as でリネームする
use std::fmt::Result;
use std::io::Result as IoResult; // ioモジュールの Result型を IoResult にリネーム
    // これによって衝突しない

   // pub useを使って名前を再公開する
    // use で要素スコープに持ち込んでもその要素は非公開です(持ち込んだ物自体は兄弟であれば参照可能)
    // pub use を使えば再度公開できます
    // これを再公開という
    //pub use crate::front_of_house::hosting; // 再公開
pub fn eat_at_restaurant_v() {
    hosting::add_to_waitlist(); // hosting を再公開しなければ add_to_waitlist関数は呼び出せなかった
}

   // 外部のパッケージを使う
    // 乱数を生成するのに randパッケージを使う場合は Cargo.toml の [dependencies] の下に
    //rand = "0.5.5"
    // と記載し rand を依存として追加し、use rand::Rng; でスコープに持ち込むことが出来る

    // 標準ライブラリ(std)も自分のパッケージの外部にあるクレートですが
    // 標準ライブラリは Rust に同封されているので Cargo.toml で std を使うように表記する必要はありません
    // しかしその要素をスコープに持ち込むには use を使う必要があります

   // 巨大なuseのリストをネストしたパスを使って整理する
    // 以下のように一部が同じ場合はまとめることができます
    //use std::cmp::Ordering;
    //use std::io;
    // この例では std:: が同一ですので以下のように表記できます
    //use std::{ cmp::Ordering, io };

    // 例外として以下のような場合は
    //use std::io;
    //use std::io::Write;
    // self で中括弧以前そのものを指せる
use std::io::{ self, Write };

   // glob演算子
    // そのパスで定義されている全公開要素をスコープに持ち込む場合は glob演算子( * ) をそのパスの後ろに書く
use std::collections::*;

    //✔以上で「useキーワードについてI」の参照終了。以降再び ../../src/lib/packages_crates_modules.rs で use節後参照

mod front_of_house_iii; // modキーワードで {} を使わずに ; で終わらせるとモジュールの中身をモジュールと同じ名前の別のファイルから読み込む
                        // この例では ./front_of_house_iii.rs
pub use crate::front_of_house_iii::hosting_iii;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist_iii();
}
    //✔試しに ./front_of_house_iii.rs 参照