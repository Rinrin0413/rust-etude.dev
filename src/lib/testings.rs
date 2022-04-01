#![allow(unused)]
    // 1972年のエッセイ「謙虚なプログラマ」にてエドガー・ダイクストラ氏は以下のように述べている
    // 「プログラムのテストは、バグの存在を示すには非常に効率的な手法であるが、 バグの不在を示すには望み薄く不適切である」
    // これは テストをするなというわけではない
    
    // プログラムの正当性は どこまで自分のコードが期待した動きをしているかです
    // Rust は プログラムの正当性に着目して設計されているが、 正当性は複雑であるため、単純に証明することはできない
    // Rust の型システムは この重荷の多くを肩代わりしてくれるが、その型システムは全種類の不当性を捕えてくれるわけではない
    // 故に Rust では 言語内で自動化されたソフトウェアテストを書くことを サポートしている

    // 例えば、渡された数値に2を足す add_two関数を書くとする
    // この関数のシグニチャは、引数に整数を取って 結果として整数を返す
    // この関数を実装しコンパイルすると、コンパイラは型チェックと借用チェックを行う
    // 例えば、整数以外の値や 無効な参照を この関数に渡していないかなどを確かめる
    // しかしながらコンパイラは 開発者の意図した動作をする関数になっているかはわからない
    // つまり、引数に2を加算していることを保証したい...
    // そんな贵樣に！ テストがあるのです。
    
    // 例えば add_two関数に3を渡した際、戻り値は5であると主張するテストを書くことができる
    // コードを変更した際に これらのテストを走らせて、定義した正当な振る舞いが変わっていないことを確認できる
    
    // テストは複雑なスキルです
    // より良いテストの書き方を多方面から学ぶには1章だけではできないが、
    // まずはともかく Rust のテスト機構のメカニズムについて学ぶ
    // また、テストを書くのに使えるアノテーション(注釈)とマクロについて、
    // テスト実行用に提供されているオプションと動作、
    // 更にテストをユニットテストや統合テストに体系化する方法についても学ぶ
pub fn writing_tests() {
// DOC.11-1
  // テストの記述法
    // テストとは、コードが期待された動作をしていること実証する Rust の関数
    // テスト関数の本体は 典型的に以下の3つの動作を行う
    //   1. 必要なデータや状態のセットアップ
    //   2. テスト対象のコードの実行
    //   3. 結果が想定通りであることを断定(以下、アサーションと呼称)
    // このようなテストを書くために用意されている、Rust の機能を見ていく
    // これには test属性, いくつかのマクロ. should_panic属性が含まれる

  // テスト関数の構成
    // 最も単純なテストは test属性での注釈(アサーション)です
    // 属性とは、Rustコードの部品に関するメタデータ
    // 例えば、構造体定義に使用した derive属性です。 
    // 関数をテスト関数に変えるには fn の前に #[test] を付け加える
    // そんで cargo test でテストを実行したら、
    // コンパイラは test属性で注釈された関数 を走らせる為の テスト用バイナリをビルドし、
    // 各テスト関数が通過したか失敗したかを報告する
}