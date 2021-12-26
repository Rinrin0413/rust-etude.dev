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
