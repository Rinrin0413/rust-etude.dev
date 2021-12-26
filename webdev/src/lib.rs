#![allow(unused)]

mod front_end {
    pub mod display {
        pub fn show_icon() {}

        fn show_haeder() {}
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
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

    //✔以上で「パス指定やプライバシーの詳細についてI」の参照終了。以降再び ../../src/lib/packages_crates_modules.rs で path_and_privacy節後参照