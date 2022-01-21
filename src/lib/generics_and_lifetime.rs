#![allow(unused)]
    // 全てのプログラミング言語には 概念の重複を効率的に扱う道具がある
    // Rust において そのような道具の1つがジェネリクス /*#ジェネリクス*/
    // ジェネリクスは具体型や他のプロパティの抽象的な代役となる
    // コード記述の際、コンパイル時にジェネリクスに何が入るかを知らずに ジェネリクスの振る舞いや他のジェネリクスとの関係を表現できる
    // 関数が未知の値の引数を取って同じ処理を複数の具体的な値に対してするのと同じく、
    // 具体的な型の代わりに何かジェネリックな(型に囚われない)型の引数を取ることができる
    // 実際 過去に Option<T>, Vec<T>, HashMap<K, V>, Result<T, E> などを使ってきた

    // 今回は 独自の型, 関数, メソッドをジェネリクスとともに定義する方法を学ぶ
    // まず関数を抽出して コードの重複を減らす方法を確認する
    // 次に同じテクニックを活用して 引数の型が異なる2つの関数からジェネリックな関数を生成する
    // また、ジェネリックな型を構造体や enum定義で使用する方法も説明する
    // それからトレイトを使ってジェネリックな方法で振る舞いを定義する方法を学ぶ
    // ジェネリックな型にトレイトを組み合わせて、
    // ジェネリックな型を単にあらゆる型に対してではなく 特定の振る舞いの特定の型のみに制限できる
    // 最後にライフタイムについて学ぶ
    // ライフタイムとは コンパイラに参照がお互いにどう関係しているかの情報を与える一種のジェネリクス
    // ライフタイムのおかげで コンパイラが参照が有効だと確認できて 多くの場面で値を借用できる
pub fn abstract_by_fun() {
  // 関数を抽出することで重複を取り除く
    // ジェネリクスの記法を学ぶ前に 関数を抽出してジェネリックな型を使わない重複の取り除きを見てみる
    // そして このテクニックを使ってジェネリックな関数を抽出することになる
    // 重複したコードを関数にまとめること気付けるのと同じく ジェネリクスを使用できる重複コードも自ずと分かって来るでしょう
    // 以下の様な リスト内の最大値を求める短いプログラムを考えてみる
    let num_list = vec![34, 50, 25, 100, 65];
    let mut max = num_list[0];
    for n in num_list {
        if n > max { max = n; } // 現時点の max より n の方が大きかったら n に置換
    }                           // つまり常に max には走査してきた値の最大値となる
    println!("最大値: {}", max);

    // 整数のリストを変数num_list に格納して 変数max にリストの最初の数字を配置している
    // そしてリストの数字全部を走査して 走査されてきた n  が max に格納された数値よりも大きければ、
    // その変数の値を置き換える。リストの数値全てを走査した頃には max はリストの最大値を保持しているはず。今回は100になる
    // 2つの異なるリストから別々に最大値を取得するには 上記コードを複製して2箇所で同じロジック(機構)を使用できる
    let num_list_ii = vec![34, 50, 25, 100, 65];
    let mut max = num_list_ii[0];
    for n in num_list_ii {
        if n > max { max = n; }
    }
    println!("最大値II: {}", max);

    let num_list_iii = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut max = num_list_iii[0];
    for n in num_list_iii {
        if n > max { max = n; }
    }
    println!("最大値III: {}", max);

    // このコードは正常に動くが間違いも起きやすい上に頭も悪そう
    // また コードを変更したい時に複数箇所を更新することになりかねない
    // この重複を排除するには 引数に与えられたリストに対し処理を行う関数を定義して抽象化できる
    // この解決策によってコードがより分かりやすく リストの最大値を探すという概念を抽象的に表現できる
    fn largest(list: &[i32]) -> i32 {
        let mut max = list[0];
        for &item in list.iter() {
            if item > max { max = item; }
        }
        max
    }

    let num_list_iv = vec![34, 50, 25, 100, 65];
    let num_list_v = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
    println!("最大値IV: {}", largest(&num_list_iv));
    println!("最大値V: {}", largest(&num_list_v));

    // largest関数にある引数list は 関数に渡す可能性のある あらゆる i32値の具体的なスライスを示している
    // 結果的に 関数呼出の際 コードは渡した特定の値に対して走る

    // 以下が　あの頭の悪いコードを上記コードに変更するのに要したステップ
    //  1. 重複したコードを見つける
    //  2. 重複したコードを関数本体に抽出して コードの入力と戻り値を関数シグニチャで指定
    //  3. 重複した2つの処理を関数で行うように更新する

    // 次はこの同じ手順をジェネリクスでも踏んで異なる方法でコードの重複を減らす
    // 関数本体が特定の値ではなく抽象的(曖昧な)なlistに対して処理できたのと同じく、
    // ジェネリクスは抽象的な型(つまりあらゆる型)に対して処理するコードを実装できる
    // 例えば関数が2つあるとする。1つは i32値のスライスから最大の要素を探し、う1つは char値のスライスから最大要素を探す
    // この重複はどう排除するのでしょうか。答えを見つけましょう
}

pub fn generics() {
      // ジェネリックなデータ型
    // 関数シグネチャや構造体等の要素の定義を生成するのにジェネリクスを使える
    // それは更に他の多くの具体的なデータ型と使える。ジェネリクスで関数, 構造体, enum, メソッドを定義する方法を見てみる
    // それから ジェネリクスがコードのパフォーマンスに与える影響についても学ぶ

   // 関数定義では
    // ジェネリクスを使う関数を定義する際 通常は関数のシグニチャにジェネリクスを配置する
    // そうするとコードがより柔軟になり コードの重複を阻止しながら関数の呼出元により多くの機能を提供できるs
    // 以下ははどちらもスライスから最大値を探す2つの関数を示している
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest { largest = item; }
        }
        largest
    }
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest { largest = item; }
        }
        largest
    }
    let num_list_vi = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest num is {}", largest_i32(&num_list_vi)); //< The largest num is 100
    println!("The largest char is {}", largest_char(&char_list)); //< The largest char is y>

    // largest_i32関数は i32スライスから最大の値を探す関数
    // largest_char関数じゃ charスライスから最大の char を探す関数
    // 各関数には重複したコードがあるので ジェネリックな型引数を導入して1つ関数にまとめる

    // これから定義する関数の引数を抽出するには 型引数(各型の引数)に名前をつける必要がある
    // 型引数の名前にはどんな識別子も使えるが T を使う
    // というのも Rust の型の命名規則がキャメルケース
    // "type" の省略として T が多くの Rustacean に採用されている

    // 関数で引数を使うとき シグニチャでその引数を宣言する必要があるのと同じく、
    // 型引数名を関数シグネチャで使う際は 使う前に型引数名を宣言する必要がある
    // ジェネリックな largest_ii関数を定義するために、
    // 型宣言を山カッコ `<>` 内で行い それを関数名と引数リストの間に配置する
    //fn largest_ii<T>(list: &[T]) -> T {

    // この定義は以下のように解読する
    // 「largest_ii関数は なんらかの型 T に関してジェネリックである」と
    // この関数には list という引数があり これは型T の値のスライス
    // largest_ii関数は同じ T型の値を返す

    // 以下は シグネチャにジェネリックなデータ型を使用して largest_ii関数を定義している
    // このコードは i32値か char値のどちらかで呼べる方法も示している
    // ただしこれはまだコンパイルできないことに注意
    /*
    fn largest_ii<T>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest { largest = item; }
        }
        largest
    }
    */

    // コンパイルすると以下のエラーが出るはず
    /*
    error[E0369]: binary operation `>` cannot be applied to type `T` // 訳: 2項演算`>`は、型`T`に適用できない
     --> src/main.rs:X:X
      |
    X |         if item > largest { largest = item; }
      |            ^^^^^^^^^^^^^^
      |
      = note: an implementation of `std::cmp::PartialOrd` might be missing for `T` // 訳: `std::cmp::PartialOrd`の実装が`T`に対して存在しない可能性がある
    */

    // 注釈(note)が std::cmp::PartialOrd について話している
    // これは トレイト。トレイトについては次の章で学ぶ
    // とりま、このエラーは largest_ii の本体は、「 T がなりうる全ての可能性のある型に対して動作できない」と言っている
    // 本体で型T の値を比較したいが これでは「値が順序付け可能な型」でしか使えない
    // 比較を可能にする為に std には型に実装できる std::cmp::PartialOrdトレイトがある
    // ジェネリックな型が特定のトレイトを持つことを指定する方法はいずれ学ぶ
    // 先にジェネリックな型引数を使用する他の方法を考える

   // 構造体定義では
    // 構造体を定義することで <>記法で1つ以上のフィールドにジェネリックな型引数を使うこともできる
    // 以下は Pos<T>構造体を定義して あらゆる型の x,y座標を保持する方法を示している
    struct Pos<T> { x: T, y: T, }
    let pos_int = Pos { x: 5, y: 10 }; // 整数型でのインスタンス生成
    let pos_float = Pos { x: 1.0, y: 4.0 }; // 浮動小数点数でのインスタンス生成

    // 構造体定義でジェネリクスを使用する記法は 関数定義のものと似ている
    // まず山カッコ内に型引数の名前(T)を構造体名の直後に宣言する
    // すると 本来具体的なデータ型を記述する構造体定義の箇所に ジェネリックな型を使用できる

    // ジェネリックな型を1つだけ使って Pos<T> を定義したので、
    // この定義は「Pos<T>構造体がなんらかの型T に関して、ジェネリックである」と言える

    // ただし その型が何であれ、x, y のフィールドは両方その同じ型になっている必要がある
    // x, y が両方ジェネリックだが 異なる型になり得る Pos構造体を定義するには、
    // 複数のジェネリックな型引数を使用すると良い
    // 例えば以下では 型T と U に関してジェネリックにして x が型T で y が型U になる
    struct PosII<T, U> { x: T, y: U, }
    let pos_ii_int = PosII { x: 5, y: 10 }; // 数値のみや、
    let pos_ii_float = PosII { x: 1.0, y: 4.0 }; // 浮動小数点数のみにも対応できる上に、
    let pos_ii_blend = PosII { x: 5, y: 4.0 }; // 型を混合できる

    // いくらでもジェネリックな型引数を使えるが 使いすぎるとコードが読みづらくなる
    // より多くのジェネリックな型が必要な時は コードの小分けが必要
}