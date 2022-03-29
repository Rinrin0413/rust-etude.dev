## Étude d'Rust.

    Rust のドキュメントの板書です。  
    Rinrin の自己解釈で溢れているの注意  
    活用してもいいですが情報の誤字や間違い等の責任は一切負いません。

贵樣も Rustacean(甲殻類) |こなリなさい.

<details>
    <summary>ブックマーク</summary>
    <div>

- [板書](./src/)
    - [メイン板書](./src/main.rs)
        - [データ型](./src/lib/data_types.rs)
        - [フロー制御](./src/lib/flow_control.rs)
        - [所有権](./src/lib/ownership.rs)
        - [構造体](./src/lib/structure.rs)
        - [列挙子](./src/lib/enm_mch_iflet.rs) / [match式](./src/lib/enm_mch_iflet.rs#L88) / [if-let記法](./src/lib/enm_mch_iflet.rs#L168)
        - [パッケージ / クレート](./src/lib/packages_crates_modules.rs) / [モジュール](./src/lib/packages_crates_modules.rs#L41)
            - [テストライブラリwebdev](./webdev/src/)
        - [コレクション](./src/lib/collections.rs)( [vector](./src/lib/collections.rs#L14) / [strings](./src/lib/collections.rs#L113) / [hash-maps](./src/lib/collections.rs#L302) )
            - [統計学モジュール](./src/lib/collections.rs#L430)
            - [ピッグ・ラテンモジュール](./src/lib/collections.rs#L496)
        - [エラー処理](./src/lib/error_handl.rs)
        - [関数でのコード抽象化](./src/lib/generics_and_traits.rs) / [ジェネリクス](./src/lib/generics_and_traits.rs#L84) / [トレイト](./src/lib/generics_and_traits.rs#L300)
        - [ライフタイム](./src/lib/lifetime.rs)
        - [自動テスト](./src/lib/testings.rs)

<br />

- [自己解釈 Markdowns](./assets/md/jp_docs/)
    - [日本語入門書の自己解釈](./assets/md/jp_docs/0.導入.md)

    </div>
</details>

<br />

> Rust Documents etc...

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja)

- [Rust By Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja)
- [std - Rust](https://doc.rust-lang.org/stable/std)

- [Rustの日本語ドキュメント](https://doc.rust-jp.rs)

- [Rust裏本](https://doc.rust-jp.rs/rust-nomicon-ja)

<br />

[![MIT](https://img.shields.io/github/license/Rinrin0413/rust-etude.dev?color=%23A11D32&style=for-the-badge)](./LICENSE)