#![allow(unused)]
    // Rust はエラー処理においても优秀です
    // ソフトウェアにおいてエラーは生きている証し
    // なので Rust には何かがおかしくなる場面を扱う機能がたくさんある
    // 様々な場面でコンパイラはプログラマに エラーの可能性を知ってコンパイルが通るまでに対応をできるように要求してくる
    // それによりエラーを発見してコードをリリースする前に適切に対処出来ていることを確認することでプログラムを頑健なものにできる

    // Rust ではエラーは大きく分けて2種類ある
    // 回復可能と回復不能なエラーです
    // ファイルが見つからない等の回復可能なエラーでは 問題をユーザに報告し処理を再試行することができる
    // 回復不能なエラーは配列の境界を超えた箇所へのアクセスなどでバグの兆候

    // 多くの言語ではこの2種のエラーを区別することはあまりない
    // 例外などの機構を使用して同様に扱う
    // Rust には例外が存在しない代わりに回復可能なエラーには Result<T, E>値がある
    // プログラムが回復不能なエラーに遭遇した際に実行を中止するpanic!マクロがある
    // このではまず panic! の呼び出しについて学び、それから Result<T, E>を戻り値にする方法を学ぶ
    // 加えて エラーからの回復を試みるか、実行を中止するか決定する際に 考慮すべき事についても学ぶ
pub fn panic() {
   // panic!で回復不能なエラー
    // 時としてコードで悪いことは起こる。そしてそれに対してできることは何もない
    // 贵樣ばどラずゑこともできなぃ:()
    // このような場面の時の為に panic!マクロが用意されている
    // panic!マクロが実行されると プログラムは失敗のメッセージを表示し スタックを巻き戻し掃除して 終了する
    // これがよく起こるのは何らかのバグが検出された時で、我々はどうエラーを処理すればいいかはっきりしない
}