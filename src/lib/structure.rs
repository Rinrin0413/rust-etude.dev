#![allow(unused)] // 未使用変数とかの許可定期
pub fn structure() {
  // 構造体を定義し、インスタンス化する
    // 構造体はタプルと似ている
    // タプルと同じくそれぞれ型の異なる値を持つこともできる
    // 違いとしては各データ片に名前が付くので値の意味が把握しやすい
    // JSON や辞書型に似たようなもの
    // これによってタプルとは違いインデックスを使わずに値にアクセスできる

    // 以下のように構造体を作ることができる
    //``` struct <構造体名> { <フィールド> }
    // フィールドとは以下のようなもの
    //``` <データ片名>: <値>,
    // 構造体名は先頭が大文字
    struct Kachik {
        species: String,
        price: u64,
        age: u32,
        is_alive: bool,
    }
    // この構造体は家畜の 種族, 売値, 年齢, 生死 が保存できる
    // ここではまだ構造体を定義するだけなので値などは入れないが型は指定する
    // ここで定義したのは空っぽの棚のようなもの

    // 構造体を使用するにはフィールドに具体的な値を入れて構造体のインスタンスを生成する
    // 先程定義した棚に物(値)を入れて変数に代入するようにしてインスタンス化する
    let mut pochi = Kachik {
        species: String::from("dog"),
        price: 140_000,
        age: 3,
        is_alive: true,
    };

    // 構造体の中身にアクセスするには以下のように記載する
    //``` <変数名>.<key>
    println!("このポチは{}元です", pochi.price); //< このポチは140000元です

    // 可変変数なら変更も可能
    pochi.price = 67_000_000;
    println!("このポチは{}元になりました", pochi.price); //< このポチは67000000元になりました

    // 関数内でインスタンス化するにはこの通り
    fn build_kachik(spc:String, prc:u64, age:u32, is_alv:bool) -> Kachik {
        Kachik {
            species: spc,
            price: prc,
            age: age,
            is_alive: is_alv,
        }   
    }
    let mowmow = build_kachik(String::from("cow"), 6_390_000, 9, true);
    println!("モーモーは {} です", mowmow.species);
    
   // フィールドと変数が同名の時にフィールド初期化省略記法を使う
    // 初期化省略記法という記法を使うと更に短く書くことができる
    fn build_kachik2(species:String, price:u64, age:u32, is_alive:bool) -> Kachik {
        Kachik { species,
            price,
            age,
            is_alive,
        }
    }

   // 構造体更新記法で他のインスタンスからインスタンスを生成する
    // 一部のフィールドの値が他のインスタンスと同じ場合、構造体更新記法が有効
    // 設定されなかったフィールドに別のインスタンスのフィールドをそのまま代入する
    let cowcow = Kachik {
        price: 3_200_000,
        is_alive: false,
        ..mowmow // 値段と生死以外は mowmowインスタンスのものを使うように宣言
    };

   // 構造体更新記法で他のインスタンスからインスタンスを生成する
    // フィールドに名前のない タプル構造体 というものもある
    struct RGB(f64, f64, f64);
    let test_color = RGB(1.0, 0.5, 0.3);
    // しかし異なるタプル構造体のインスタンスは、もし構造体内のフィールドが同じ型だとしても独自の型となる
    // でも以下のような計算はできる模様
    let test_boo = test_color.1 + 1.0; // 1.5

   // フィールドのないユニット様(よう)構造体
    // そして何故かフィールドのない構造体も作れる
    struct Unitkozotay();
    // 使用場面はいずれ勉強する
}