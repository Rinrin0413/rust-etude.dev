## これは何?

Rust勉強してたときの板書の一部  
勝手な自己解釈で溢れているの注意

<details>
    <summary>もくじ</summary>

- [板書](./src/)
    - [メイン板書](./src/main.rs)
        - [データ型](./src/lib/data_types.rs)
        - [フロー制御](./src/lib/flow_control.rs)
        - [所有権](./src/lib/ownership.rs)
        - [構造体](./src/lib/structure.rs)
        - [列挙子](./src/lib/enm_mch_iflet.rs) / [match式](./src/lib/enm_mch_iflet.rs#L88) / [if-let記法](./src/lib/enm_mch_iflet.rs#L168)
        - [パッケージ / クレート](./src/lib/packages_crates_modules.rs) / [モジュール](./src/lib/packages_crates_modules.rs#L41)
            - [テストライブラリwebdev](./webdev/)
        - [コレクション](./src/lib/collections.rs)( [vector](./src/lib/collections.rs#L14) / [strings](./src/lib/collections.rs#L113) / [hash-maps](./src/lib/collections.rs#L302) )
            - [統計学モジュール](./src/lib/collections.rs#L430)
            - [ピッグ・ラテンモジュール](./src/lib/collections.rs#L496)
        - [エラー処理](./src/lib/error_handl.rs)
        - [関数でのコード抽象化](./src/lib/generics_and_traits.rs) / [ジェネリクス](./src/lib/generics_and_traits.rs#L84) / [トレイト](./src/lib/generics_and_traits.rs#L300)
        - [ライフタイム](./src/lib/lifetime.rs)
        - [テスト機能](./src/lib/testings.rs)
            - [テストライブラリadder](./adder/)

<!-- <br />

- [自己解釈シリーズ](./assets/md/jp_docs/)
    - [日本語入門書](./assets/md/jp_docs/0.導入.md) -->

</details>

## 参考文献とか

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja)
- [Rust By Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja)
- [Rust ツアー - Let's go on an adventure!](https://tourofrust.com/00_ja.html)
- [std - Rust](https://doc.rust-lang.org/stable/std)
- [Rustの日本語ドキュメント](https://doc.rust-jp.rs)
- [Rust裏本](https://doc.rust-jp.rs/rust-nomicon-ja)
