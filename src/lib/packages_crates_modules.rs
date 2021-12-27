#![allow(unused)] // 未使用変数とかの許可定期

    // 関係した機能をまとめたり異なる特徴を持つコードを分割すると、目的のコードを見つけたり機能を変更したりする際に便利です
    // パッケージは複数のバイナリクレートからなり、ライブラリクレートを1つもつこともできます
    // パッケージが肥大化したら一部を抜き出して分離したクレートにし、外部依存とするのもよい

    // パス : 要素(例えば構造体や関数やモジュール)に名前をつける方法
    // モジュール & use : これを使うことでパスの構成, スコープ, 公開するか否かを決定できます
    // クレート : ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群
    // パッケージ : クレートをビルドし、テストし、共有することができるCargoの機能

// packages_and_crate {
   // パッケージとクレート
    // クレートはバイナリかライブラリのどちらかです
    // クレートルート(crate root) とはコンパイラの開始点となりクレートのルートモジュールを作るソースファイルとなります
    // パッケージはある機能群を提供する1つ以上のクレートです
    // パッケージはそれらのクレートをどのようにビルドするかを説明するファイル( Cargo.toml )を持っています
    // そしてパッケージは0か1個のライブラリクレートを持っていないといけないが、バイナリクレートはいくらでも持って良い
    // 少なくとも1つのクレート(libクレbinクレどちらでもよい) を持っていないといけない

    // 以下のプロジェクト構成の場合
    // ---------------
    // shogee
    //  ├ src
    //  |  └ main.rs
    //  └ Cargo.toml
    // ---------------
    // このパッケージには  src/main.rs しか含まれていなくて shogee というバイナリクレートしか持っていないことになる
    // src/lib.rs を持っていたらクレートは2つになり、どちらもパッケージと同じ名前を持つlibクレとbinクレとなります
    // そしてファイルを src/bin配下に置くことでパッケージは複数のバイナリクレートを持つことができる
    // それぞれのファイルが別々のバイナリクレートになる

    // クレートは関連した機能を1つのスコープにまとめてその機能が複数のプロジェクト間で共有しやすいようにします
    // 例えば randクレート は乱数を生成する機能を提供します
    // randクレート を自分のプロジェクトのスコープに持ち込むことでこの機能をプロジェクトで使うことができます
    // また、クレートで名前の衝突も防ぐことができます
    // randクレート の 構造体Rang とは別にプロジェクトで Rang という構造体を作ることができます
    // コンパイラは rand::Rang と Rang で別々に考えるからです
// }

// def_mods {
   // モジュールを定義して、スコープとプライバシーを制御する
    // モジュールとその他のモジュールシステムの要素
    // つまり、要素に名前をつけるためのパス, パスをスコープに持ち込むuseキーワード, 要素を公開するpubキーワード, について学びます
    // また、asキーワード, 外部パッケージ, glob演算子についても学びます

    // モジュールはクレート内のコードをグループ化して可読性と再利用性を上げるのに役に立ちます
    // モジュールは要素のプライバシーも制御できます
    // プライバシーとは要素がコードの外側で使える(公開 public) のか内部でのみ使う外部では使えない(非公開 private) のかです

    // 今回はweb開発の仕組みで考えてみます
    // フロントエンドとバックエンドがありますね
    // 以下でライブラリを作成します
    //>>> cargo new --lib webdev
    // 以下が webdev/src/lib.rs です
    //mod front_end {
    //    mod display {
    //        fn show_icon() {}
    //
    //        fn show_haeder() {}
    //    }
    //    mod animation {
    //        fn rotate_icon() {}
    //
    //        fn scale_up_header_in_hover() {}
    //    }
    //}
    // モジュールはmodキーワードを書き次にモジュールの名前を指定することで定義できます
    // モジュール内に他のモジュールを置くこともできます
    // その他の要素の定義も置くことができます ex:構造体, enum, 定数, トレイト, 関数 etc...

    // 以下はモジュールツリーです
    // crate
    // └── front_end
    //     ├── display
    //     │   ├── show_icon
    //     │   └── show_haeder
    //     └── serving
    //         ├── rotate_icon
    //         └── scale_up_header_in_hover
    // この場合 display と serving は兄弟の関係にあります
    // そして show_icon は display の子, display は show_icon の親となります
// }

// path_and_privacy {
  // モジュールツリーの要素を示すためのパス
    // モジュールツリー内の要素を見つける為にパスを使う
	// パスは2つの形を取ることができる
	// 絶対パス : クレートの名前か crate という文字列を使うことでクレートルートからスタートできる
	// 相対パス : self, super または今のモジュール内の識別子を使うことで現在のモジュールからスタートできる
	// 後に1つ以上の識別子が `::` で仕切られて続く

	// show_icon関数を呼び出すには show_icon のパスが必要
	//✔パス指定やプライバシーの詳細については ../../webdev/src/lib.rs の browse関数定義以下参照
// }

// use {
  // useキーワードでパスをスコープに持ち込む
    // パスはどうしても長くなって今う場合がある
    // 繰り返し書くのも良いとは思えない
    // useキーワードでパスをスコープに持ち込めば解決する
    //✔以降は ../webdev/src/lib.rs の front_of_houseモジュール定義参照
// }