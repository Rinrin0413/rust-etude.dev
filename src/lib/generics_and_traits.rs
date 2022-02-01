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

   // enum定義では
    // 構造体と同じく 列挙子にジェネリックなデータ型を保持する enum を定義できる
    // std が提供している Option<T> enum をもう一度見てみる
    enum Option<T> { Some(T), None, }

    // ここでは Option<T> は型T に関してジェネリックで2つの列挙子のある enum となる
    // その列挙子は型T の値を保持する Some と値を何も保持しない None
    // Option<T> enum を使うことで オプショナルな値があるという抽象的な概念を表現できて、
    // Option<T> はジェネリックなので 型に関わらず使用できる
    // enum も複数のジェネリックな型を使える。Result<T> が一例です
    enum Result<T, E> { Ok(T), Err(E), }

    // Result enum は2つの型T, E に関してジェネリックで 2つの列挙子がある
    // 型T の値を保持する Ok と 型E の値を保持する Err です
    // この定義により Result enum を 成功する(型Tの値を返す)か 失敗する(型Eのエラーを返す) のどちらかの処理ができる
    // あらゆる箇所に使用するのが便利になる。
    // 自分のコードで、保持している値の型のみが異なる構造体や enum定義があったら 代わりにジェネリックな型を使うと良い


   // メソッド定義では
    // 定義にジェネリックな型を使うメソッドを 構造体や enum に実装することもでる
    // 以下は先ほど定義した Pos<T>構造体に x というメソッドを実装したもの
    //struct Pos<T> { x: T, y: T, } // さっき定義したやつ
    impl<T> Pos<T> { // Pos<T> であらゆる型を持つ Pos に実装することを示す
        fn x(&self) -> &T { &self.x } // 関連関数x
        fn y(&self) -> &T { &self.y } // 関連関数y
    }
    let pos = Pos { x: 5, y: 10 };
    println!(
        "x: {}\ny: {}", 
        pos.x(), 
        pos.y()
    ); //< x: 5\ny: 10

    // ここで フィールド x や y のデータへの参照を返す xメソッドと yメソッドを Pos<T> に定義した
    // impl の直後に T を宣言している。これで Pos<T>型にメソッドを実装していることを指定するために T を使える
    // implの後に T をジェネリックな型として宣言することで、コンパイラは Pos の山カッコ内の型が 具体的な型ではなくジェネリックな型であることを認識できる
    // 例えば ジェネリックな型を持つ Pos<T>インスタンスではなく、
    // Pos<f32> とすれば f32を保持するもののみに実装する ということも可能
    // 以下では具体的な型f32を使用している。つまり impl の後に型を宣言する必要がない
    impl Pos<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // このコードで Point<f32> に distance_from_origin というメソッドが存在するが、
    // T が f32 ではない Point<T> の他のインスタンスにはこのメソッドが定義されないことを意味する。
    // 例えば i32 で生成されたインスタンスには使用できない
    // このメソッドは この位置が座標(0.0, 0.0) からどれだけ離れているかを測定して 浮動小数点数にのみ利用可能な数学的処理をしている

    // 構造体定義のジェネリックな型引数は 必ずしもその構造体のメソッドシグニチャで使用するものと同じにはならない
    // 例えば以下は 先ほどののPos_ii<T, U>に mixupメソッドを定義している
    // このメソッドは 他の Pos を引数として取る。この引数は mixup を呼び出している self の Pos とは異なる型の可能性がある
    // このメソッドは 型T の self の Pos の x値と渡した型W Pos の y値から新しい Posインスタンスを生成する
    impl<T, U> PosII<T, U> {
        fn mixup<V, W>(self, other: PosII<V, W>) -> PosII<T, W> {
            PosII { x: self.x, y: other.y, } // 連結元(self)の x座標と引数に入れた別の PosIIインスタンス(other)の y座標で、
                                             // 新しい PosIIインスタンスを生成する
        }
    }
    let pos_iia = PosII { x: 5, y: 10.4 };
    let pos_iib = PosII { x: "Hello", y: 'c'};
    let pos_iic = pos_iia.mixup(pos_iib); // PosII { x: 5, y: 'c' }
    println!("x: {}\ny: {}", pos_iic.x, pos_iic.y); //< x: 5\ny: c

    // x に i32, y に f64 を持つ Posインスタンスpos_iia を定義して、
    // x に文字列スライス, y に char を持つ Posインスタンスpos_iibも定義すた
    // 引数に pos_iib を入れて pos_iia に mixup を呼び出すと、
    // x が pos_iia由来の i32、y が pos_iib由来の char となるインスタンスが返される

    // ジェネリックな引数V,W は `fn mixup` の後に宣言されている。何故ならこのメソッドにしか関係ないから

   // ジェネリクスを使用したコードのパフォーマンス
    // ジェネリックな型引数を使用すると パフォーマンスが低下するのでは と思うかもしれない
    // 幸い Rust では ジェネリクスを使っても実行が遅くならないように実装している
    // コンパイラはこれを ジェネリクスを使用しているコードの単相化をコンパイル時に行うことで達成している
    // 単相化(monomorphization)は コンパイル時に使われている具体的な型を入れることで、ジェネリックなコードを特定のコードに変換する過程のこと。
    // この過程でコンパイラは ジェネリックなコードが呼び出されている箇所全てを見て、 
    // ジェネリックなコードが呼び出されている具体的な型のコードを生成する
    // std の Option<T> enum を使用する例で これが動作する方法を見る
    let integer = Some(5); //: std::option::Option<i32>
    let float = Some(5.0); //: std::option::Option<f64>

    // このコードをコンパイルすると 単相化が行われる
    // その過程で コンパイラは Option<T> のインスタンスに使われた値(ここでは5や5.0)を読み取って、
    // 2種類の Option<T> を識別する。一方は i32 で もう片方は f64 となる
    // そのように、コンパイラは Option<T> のジェネリックな定義を Option_i32 と Option_f64 に展開して、
    // それにより ジェネリックな定義を特定の定義と置き換える
    // 単相化されたバージョンのコードは 以下のようになる
    // ジェネリックな Option<T> が コンパイラが生成した特定の定義に置き換えられている
    /*
    enum Option_i32 { // コンパイラの特権なのか型名にアンダーバーが使える
        Some(i32),
        None,
    }
    enum Option_f64 {
        Some(f64),
        None,
    }
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
    */

    // Rust では ジェネリックなコードを各インスタンスで型を指定したコードにコンパイルするので、
    // ジェネリクスを使っても実行時コストを払うことはない。コードを実行すると それぞれの定義を手作業で複製した時のように振る舞う
    // 単相化の過程により Rust のジェネリクスは実行時に究極的に効率的になる
}

pub fn traits() {
 // トレイト: 共通の振る舞いを定義する
    // トレイトは特定の型に存在し 他の型と共有できる機能について Rustコンパイラに知らせる
    // トレイトを使用すると 共通の振る舞いを抽象的に定義できる
    // トレイト境界を使用すると あるジェネリックが特定の振る舞いをもつ あらゆる型になり得ることを指定できる
    // 違いはあるものの トレイトは他の言語でよくインターフェイスと呼ばれる機能に類似している
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // ここでは traitキーワード それからトレイト名を使用してトレイトを定義している
    // 今回の場合トレイト名は Summary です。波括弧の中にこのトレイトを実装する型の振る舞いを記述するメソッドシグニチャを定義し、
    // 今回の場合は、`fn summarize(&self) -> String` です。
    //　メソッドシグニチャの後に 波括弧内ではなくセミコロンを置いている
    // このトレイトを実装する型はそれぞれ メソッドの本体に独自の振る舞いを提供することになる
    // コンパイラにより Summaryトレイトを保持するあらゆる型に、
    // このシグニチャと全く同じメソッド summarize が定義されていることが強制される
    // トレイトには 複数のメソッドを含むこともできる。メソッドシグニチャは行ごとに並べられて 各行はセミコロンで終わる

   // トレイトを型に実装する
    // 今や Summaryトレイトを使って目的の動作を定義できたので、メディア アグリゲータでこれを型に実装できる
    // 以下は Summaryトレイトを NewsArticle 構造体上に実装したもので、ヘッドライン, 著者, 地域情報を使って summarize の戻り値を作っている
    // Tweet構造体に関しては、ツイートの内容が既に280文字に制限されていると仮定して ユーザー名の後にツイートのテキスト全体が続くものとして summarize を定義する
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    /*impl Summary for NewsArticle { // あとで邪魔になるので無効化
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }*/
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} さんより\n{}", self.username, self.content) // format!マクロの返り値はString
        }
    }

    // 型にトレイトを実装することは 普通のメソッドを実装することに似ている
    // 違いは impl の後に 実装したいトレイトの名前を置き forキーワード 更にはトレイトの実装対象の型の名前を指定すること
    // implブロック内に トレイト定義で定義したメソッドシグニチャを置く
    // 各シグニチャの後には セミコロンではなく波括弧を使用し、メソッド本体に特定の型のトレイトのメソッドに欲しい特定の振る舞いをいれる
    // トレイトを実装後 普通のメソッド同様に NewsArticle や Tweet のインスタンスに対してこのメソッドを呼び出せる
    let tweet_0 = Tweet {
        username: String::from("龴亻クソソ㇇ㇳ"),
        content: String::from("暇だしwindows12出すわw(適当)\nwin11の方はサポート終わりで(笑)"),
        reply: true,
        retweet: true,
    };
    println!("1件の新着のツイート: {}", tweet_0.summarize()); //< 1件の新着のツイート: 龴亻クソソ㇇ㇳ さんより\n暇だしwindows12出すわw(適当)\nwin11の方はサポート終わりで(笑)

    // 先ほど Summaryトレイトと NewArticle, Tweet型を同じファイルに定義したので 全て同じスコープにあった
    // このファイルを aggregator と呼ばれるクレート専用にして、
    // 他の人が自分のクレートの機能を活用して自分のライブラリのスコープに定義された構造体に Summaryトレイトを実装したいとする
    // まず トレイトをスコープに取り込む必要がある。`use aggregator::Summary;` とすれば自分の型に Summaryトレイトを実装することが可能になる
    // Summaryトレイトは 他のクレートが実装するために公開トレイトである必要があり、
    // ここでは traitキーワード前に pubキーワードを置いた

    // トレイト実装で注意すべき制限の1つとして、トレイトか対象の型が自分のクレートに固有(local)な時のみ、型に対しトレイトを実装できるということ
    // 例えば Displayトレイトのような標準ライブラリのトレイトを aggregatorクレートの機能の一部として、Tweet などの独自の型に実装できる
    // 型Tweet が aggregatorクレートに固有(local)だから故。 また Summaryトレイトを aggregatorクレートで Vec<T> に対して実装することもできる
    // トレイトSummary は aggregatorクレートに固有だから

    // しかし 外部のトレイトを外部の型に対して実装することはできない
    // 例として aggregatorクレート内で Vec<T> に対して Displayトレイトを実装することはできない
    // Display と Vec<T> は標準ライブラリで定義されているので aggregatorクレートに固有ではない故。
    // この制限はコヒーレンス(coherence) 特に孤児のルール(orphan rule)と呼ばれるプログラムの特性の一部で、
    // 親の型が存在しないためにそう命名された。この規則により他の人のコードが自分のコードを壊したり その逆が起きないことを保証してくれる
    // この規則がなければ 2つのクレートが同じ型に対して同じトレイトを実装できてしまい、 
    // コンパイラはどちらの実装を使うべきか分からなくなる

  // デフォルト実装
    // 時として 全ての型の全メソッドに対して実装を要求するのではなく、
    // トレイトの全てあるいは一部のメソッドに対して デフォルトの振る舞いがあると有用
    // さすれば 特定の型にトレイトを実装する際に 各メソッドのデフォルト実装を保持するかオーバーライドするか選べる
    // 以下はトレイト定義で メソッドシグニチャだけを定義するのではなく、
    // Summaryトレイトの summarizeメソッドにデフォルトの文字列を指定している
    pub trait SummaryII {
        fn summarize(&self) -> String { String::from("(続きを読む)") }
    }

    // 各型で独自の実装を定義するのではなく デフォルト実装を利用して NewsArticle のインスタンスをまとめるには、
    // `impl SummaryII for NewsArticle {}` と空のimplブロックを指定する
    // もはや NewsArticle に明示的に summarizeメソッドを定義していないが、
    // デフォルト実装を提供していて NewsArticle は Summaryトレイトを実装すると指定している
    // そのため NewsArticle のインスタンスに対して summarizeメソッドを呼び出すことができる
    impl SummaryII for NewsArticle {}
    let article = NewsArticle {
        headline: String::from("悲报:龴亻クソソ㇇ㇳ、windows12を暇潰しで開発"),
        location: String::from("Earth"),
        author: String::from("Tom"),
        content: String::from(
            "龴亻クソソ㇇ㇳが先日windows12を暇潰しでリリース。\
            これによってwindows11は放棄されるとのこと",
        ),
    };
    println!("新着のニュース[{}]\n{}", article.headline, article.summarize()); //< 新着のニュース[悲报:龴亻クソソ㇇ㇳ、windows12を暇潰しで開発]\n(続きを読む)

    // Summaryトレイトの summarizeメソッドにデフォルト実装を用意しても、構造体Tweet の Summary実装を変える必要はない
    // 理由はデフォルト実装をオーバーライドする記法はデフォルト実装のないトレイトメソッドを実装する記法と同じ故。
    // デフォルト実装は 自らのトレイトのデフォルト実装を持たない他のメソッドを呼び出すことができる
    // こうすれば トレイトは多くの有用な機能を提供しつつ実装者は僅かな部分だけ指定すればよい
    // 例えば Summaryトレイトを実装者が内容を実装しなければならない summarize_authorメソッドを持つように定義し、
    // それから summarize_authorメソッドを呼び出すデフォルト実装を持つ summarizeメソッドを定義することもできる
    pub trait SummaryIII {
        fn summarize_author(&self) -> String;
        fn summarize(&self, arg:&str) -> String {
            format!("{}氏の{}の続きを読む", self.summarize_author(), arg)
        }
    }

    // このバージョンの SummaryIII を使用するために型にトレイトを実装する際、
    // 実装する必要があるのは summarize_author のみとなる
    pub struct TweetII { pub username: String, pub content: String, pub reply: bool, pub retweet: bool, } // 他の summarizeメソッドと被るので新しいのを
    impl SummaryIII for TweetII {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // summarize_authorメソッド定義後 Tweet構造体のインスタンスに対して summarize を呼び出せ、
    // summarizeのデフォルト実装は今提供した summarize_author の定義を呼び出す
    // summarize_authorメソッドを実装したので追加のコードを書く必要なく SummaryIIIトレイトは summarizeメソッドの振る舞いを与えてくれた
    let tweet_1 = TweetII {
        username: String::from("龴亻クソソ㇇ㇳ"),
        content: String::from("不和 買収してくるわwww(守銭奴)"),
        reply: true,
        retweet: true,
    };
    println!("1件の新着のツイート: {}", tweet_1.summarize("ツイート")); //< 1件の新着のツイート: @龴亻クソソ㇇ㇳ氏のツイートの続きを読む

    // デフォルト実装を そのメソッドをオーバーライドしている実装から呼び出すことはできないことに注意

  // 引数としてのトレイト
    // トレイトを定義し実装する方法は分かったので、トレイトを使ってジェネリックな関数を定義する方法を考える
    // たとえば先程は NewsArticle と Tweet型に Summaryトレイトを実装した
    // ここで引数の item の summarizeメソッドを呼ぶ関数notify を定義することができる
    // ただし引数item は Summaryトレイトを実装している型であるとする
    // このようなことをするためには impl Trait構文を使うと良い
    pub fn notify(item: &impl Summary) {
        println!("速報[{}]", item.summarize());
    }

    // 引数item には具体的な型の代わりに implキーワードとトレイト名を指定している
    // この引数は指定されたトレイトを実装している型を受け付ける
    // notify関数の中では summarizeメソッドのように Summaryトレイト由来の あらゆるメソッドを呼び出すことができる
    // 私達は notify を呼び出して NewsArticle か Tweet のどんなインスタンスでも渡すことができる
    // この関数を呼び出すときに String や i32 のような他の型を渡すようなコードを引数に入れるとコンパイルできない
    // なぜなら これらの型は Summary を実装していないから

   // トレイト境界構文
    // impl Trait構文は、より長い トレイト境界(trait bound) と呼ばれる姿の糖衣構文(syntax sugar)です
    // 以下がそのトレイト境界です
    pub fn notify_ii<T: Summary>(item: &T) {
        println!("速報[{}]", item.summarize());
    }

    // このより長い姿はimpl Trait構文の例と等価だが、より冗長
    // 山括弧内にジェネリック型引数の宣言を書いて 型引数の後ろにコロンを挟んでトレイト境界を置いている
    // 簡単なケースでは impl Trait構文は便利で コードを簡潔にしてくれる
    // そうでないケースの場合 トレイト境界構文を使えば複雑な状態を表現できる
    // 例えば Summaryトレイトを実装する2つのパラメータを持つような関数を考えることができる
    // impl Trait構文を使うとこのようになる
    //pub fn notify(item1: &impl Summary, item2: &impl Summary) {

    // この関数が受け取る item1 と item2 の型が異なっても良いなら impl Trai構文tは適切です
    // 両方の引数が同じ型であることを強制するには 以下のようにトレイト境界を使って実装できる
    //pub fn notify<T: Summary>(item1: &T, item2: &T) {

    // 引数である item1 と item2 の型としてジェネリックな型T を指定した
    // これにより item1 と item2 の値の具体的な型が同一でなければならない、という制約を与えている

   // 複数のトレイト境界を+構文で指定する
    // 複数のトレイト境界の指定もできる。例えば notify に summarizeメソッドに加えて item の画面出力形式(ディスプレイフォーマット)を使わせたいとする
    // その場合は notify の定義に item は Display と Summary の両方を実装していなくてはならないと指定することになる
    // これは 以下のように +構文で行うことができる `&(impl TraitI + TraitII)`
    //pub fn notify(item: &(impl Summary + Display)) {

    // +構文はジェネリック型につけたトレイト境界に対しても使える `TraitI + TraitII`
    //pub fn notify<T: Summary + Display>(item: &T) {

    // これら2つのトレイト境界(Summary, Display)が指定されていれば、
    // notify の中では summarize を呼び出すことと {} を使って item をフォーマット(画面出力)することの両方が行なえる

   // where句を使ったより明確なトレイト境界
    // 過度に多くのトレイト境界を使うことには欠点もある
    // それぞれのジェネリック型がそれぞれのトレイト境界をもつので、
    // 関数名と引数リストの間に大量のトレイト境界に関する情報を含むことがある
    // これでは関数のシグネチャが読みにくくなってしまう
    // このため Rust はトレイト境界を関数シグネチャの後の where句の中で指定するという別の構文を用意している
    // 例として、T が Displayトレイト及び Cloneトレイトの実装された型、
    // U が Cloneトレイト及び Debugトレイトの実装された型 とする
    //fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

    // where句を使うと
    /*
    fn some_function<T, U>(t: &T, u: &U) -> i32 where 
        T: Display + Clone, 
        U: Clone + Debug {
    */

    // この関数シグニチャは トレイト境界を多く持たない関数と同じく 関数名, 引数リスト, 戻り値の型が一緒になって近くにある故見やすい

  // トレイトを実装している型を返す
    // 以下のように impl Trait構文を戻り値の型の部分で使うことで 指定したトレイトを実装する型を返すことができる
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("龴亻クソソ㇇ㇳ"),
            content: String::from("win12やっぱやめるめんどい(^^)v"),
            reply: true,
            retweet: true,
        }
    }

    // 戻り値の型として impl Summary を使うことにより 具体的な型を言うことなく returns_summarizable関数は Summaryトレイトを実装している何らかの型を返す
    // 今回 returns_summarizable は Tweet を返すが、この関数を呼び出すコードはそのことを知らない

    // 実装しているトレイトだけで戻り値型を指定できることはいずれ学ぶ クロージャとイテレータを扱うときに便利
    // クロージャとイテレータの作り出す型は コンパイラだけが知っているものであったり 指定するには長すぎるものであったりする
    // impl Trait構文を使えば 非常に長い型を書くことなく ある関数は Iteratorトレイトを実装するある型を返すのだと簡潔に指定することもできる
    // ただし impl Trait構文は一種類の型を返す場合にのみ使える
    // たとえば以下のように 戻り値の型は impl Summary で指定しつつ NewsArticle か Tweet のどちらかを返すようなコードは失敗する
    /*
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from("The Pittsburgh Penguins once again are the best\n hockey team in the NHL."),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }
    */

    // NewsArticl か Tweet を返すというのは コンパイラの impl Trait構文の実装まわりの制約により許されていない
    // このような振る舞いをする関数を書く方法はいずれ学ぶ

  // トレイト境界で largest関数を修正する
    // 以下が前の節で作った largest_ii関数(コンパイルエラーで使えない)
    /*
    fn largest_ii<T>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest { largest = item; }
        }
        largest
    }
    */

    // トレイト境界で望んだ振る舞いを指定する方法が分かったので 上記の largest_ii関数の定義を修正する
    // 上記のコードを実行しようとした時 こんなエラーが出ていたはず
    /*
    error[E0369]: binary operation `>` cannot be applied to type `T`
     --> src/main.rs:X:X
      |
    X |         if item > largest { largest = item; }
      |            ---- ^ ------- T
      |            |
      |            T
      |
      = note: `T` might need a bound for `std::cmp::PartialOrd`

    error: aborting due to previous error
    */

    // 関数では `>` を使って型T の値の比較を試みた。この >演算子は std の std::cmp::PartialOrdトレイトで、
    // デフォルトメソッドとして定義されている故 largest_ii関数で T に対して利用するには、
    // T のトレイト境界に PartialOrd を指定する必要がある。シグネチャを修正してみる
    //fn largest<T: PartialOrd>(list: &[T]) -> T {

    // しかし別のエラーが降臨
    /*
    error[E0508]: cannot move out of type `[T]`, a non-copy slice // 訳: 型[T] を持つ非コピーのスライスからのムーブはできないよ）
     --> src/main.rs:X:X
      |
    X |     let mut largest = list[0];
      |                       ^^^^^^^
      |                       |
      |                       cannot move out of here // ここからムーブはできないよ！
      |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
      |                                                    // └ 訳: この T は Copyトレイトを実装してない型だよ
      |                       help: consider borrowing here: `&list[0]` // 訳: 借用するといいよ☞ &list[0]

    error[E0507]: cannot move out of a shared reference // 訳: 共有の参照からのムーブはできねぇから！
     --> src/main.rs:X:X
      |
    X |     for &item in list {
      |         -----    ^^┗^^
      |         ||
      |         |data moved here // 訳: ここでデータ ムーブしてんだろ？ わかってっから！
      |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait // 訳: T が Copy`トレイト実装してねぇ型だからこうなんだよ！
      |         help: consider removing the `&`: `item` // 訳: & 早く消せよオラァァ゛☞ item

    error: aborting due to 2 previous errors
    */

    // このエラーの鍵となるのは `cannot move out of type [T]` と `a non-copy slice` です
    // ジェネリックでない頃の largest関数は 最大の i32 か char を探すだけだった
    // i32 や char のような型はスタックに格納できるので Copyトレイトを実装している
    // しかし largest関数をジェネリックにすると、引数が Copyトレイトを実装しない型を含む可能性も出てきた
    // 結果として list[0] から値を largest にムーブできず このエラーに陥った

    // Copyトレイトを実装する型だけを呼び出したいなら T のトレイト境界に Copy を追加すればよい
    // 以下は 関数に渡したスライスの値の型が PartialOrd 及び Copy を実装する限りコンパイルできる ジェネリックな largest_ii関数の定義のなる

    fn largest_ii<T: PartialOrd + Copy>(list: &[T]) -> T {
    //             ┗ PartialOrdトレイト(比較演算子を使うため) と Copyトレイト(スタックに保存したい) のみを受け入れる
        let mut largest = list[0];
        for &item in list {
            if item > largest { largest = item; }
        }
        largest
    }

    // 使える..！
    let num_list_vii = vec![86, 32, 64, 12, 0];
    let char_list_ii = vec!['x', 'y', 'z', 'a', 'b', 'c'];

    println!("最大値i: {}", largest_ii(&num_list_vii)); //< 最大値i: 86
    println!("最大値ii:{}", largest_ii(&char_list_ii)); //< 最大値ii:z

    // もし largest_ii関数を Copyを実装する型だけに制限したくなかったら、
    // Copy ではなく Clone のトレイト境界を持たせる
    // そしたら largest_ii関数に 所有権がごと与えられるスライスの各値をクローンできる

    // しかし clone関数を使うと ヒープデータを持つ型によって より多くのヒープ確保が発生するかもしれない
    // そして 大量のデータを取り扱っていたら ヒープ確保には時間がかかることもある

    // largest_ii関数の別の実装方法は、関数がスライスの T値への参照を返すようにすること
    // 戻り値の型を T ではなく &T に変えると Clone,Copyトレイト境界は必要なって ヒープ確保も避けられる
}