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
        Kachik { 
            species,
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

pub fn ex_refactoring() {
  // 構造体を使ったプログラム例
    // 長方形の幅と高さをピクセルで指定しその面積を求めるプログラムを見て
    // リファクタリングしてみる
    // ↓ソースコード
    let width1 = 30;
    let height1 = 50;
    fn area(width: u32, height: u32) -> u32 { width * height }
    println!(
        "The area of the rectangle is {} square pixels.", 
        area(width1, height1)
    ); //< The area of the rectangle is 1500 square pixels.

   // タプルでリファクタリング
    // 幅と高さは1つにまとめる
    let rect = (30, 50);
    fn area_kay(rect:(u32, u32)) -> u32 { rect.0*rect.1 }
    println!(
        "The area of the rectangle is {} square pixels.",
        area_kay(rect)
    );
    // しかし rect.0*rect.1 では計算の意味が分かりづらい
    // タプルのインデックス 0が幅で、1が高さ ということを覚えておかなければならない
    // 他者が見たりいじったりする際も非常に分かりづらい

   // 構造体でリファクタリングし意味付けする
    // フィールドに名前を付けたいので構造体を使う
    struct Rect { width: u32, height: u32, }
    fn area_kai_ii(rect:Rect) -> u32 {
        rect.width * rect.height
    }
    let rect2 = Rect { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_kai_ii(rect2)
    );
    // やっとプログラムの意図が分かり易くなりますた

   // トレイトの導出で有用な機能を追加する
    // 処理の途中で Rectインスタンスの内容を確認したいとする
    let rect3 = Rect { width: 86, height: 32 };
    //println!("{}", rect3);
    // しかしそうはいかない
    // println!マクロには様々な整形があるが 構造体を出力する形式が複数ある
    // カンマが必要か、波かっこを出力するか、フィールドを全て表示するか、などなど
    // これでは不明確で混乱

    // println!マクロ第1引数の波括弧内に `:?` を入れてみる
    //println!("{:?}", rect3);
    // しかしながらエラー；；
    // 構造体をデバック用の情報として出力するには特別な注釈が必要故
    // 構造体定義直前に以下を置く
    //`#[derive(Debug)]
    // ではやってみる
    #[derive(Debug)]
    struct RectDebug { width: u32, height: u32, }
    let rect4 = RectDebug { width: 860, height: 320 };
    println!("{:?}", rect4); //< RectDebug { width: 860, height: 320 }

    // println!マクロ第1引数の波括弧内の `:?` を `:#?` に変えると更に整形された形になる
    println!("{:#?}", rect4);
    //< RectDebug {
    //<     width: 860,
    //<     height: 320,
    //< }

    // `:?` はバックトレイトという
}

pub fn method() {
  // メソッド記法
	// メソッドとは関数に似たもの
	// しかし構造体のように定義する
	// そして第1引数が必ず self となる

   // メソッドを定義する
   #[derive(Debug)]
   struct MthRect { wid: u32, hei: u32, } // 構造体の定義
   impl MthRect { // 構造体MthRect の文脈で関数を定義(つまりメソッドを定義)
       fn mth_area(self) -> u32 { self.wid*self.hei } // この self は構造体MthRect のインスタンスとなる
   }
   let mth_rect1 = MthRect { wid: 28, hei: 16 }; // インスタンス生成
   println!(
       "The area of the rectangle is {} square pixels.",
       mth_rect1.mth_area() // メソッド記法(インスタンスの後にピリオドを挟んでメソッドを呼ぶ)
   ); //< The area of the rectangle is 448 square pixels.

  // より引数の多いメソッド
   impl MthRect {
       fn area(self) -> u32 { self.wid*self.hei }
       fn can_hold(&self, other:MthRect) -> bool { // 別の MthRectインスタンスを受け入れる
           self.wid > other.wid && self.hei > other.hei // 第2引数に入ってきたインスタンス(の四角形)が self(の四角形)に収まるなら true を返す式
       }
   }
   let meth_rect1 = MthRect { wid: 30, hei: 50 };
   let meth_rect2 = MthRect { wid: 10, hei: 40 };
   let meth_rect3 = MthRect { wid: 60, hei: 45 };
   // can_holdメソッドに比べさせる
   println!("Can rect1 hold meth_rect2? {}", meth_rect1.can_hold(meth_rect2)); //< Can rect1 hold meth_rect2? true
   println!("Can rect1 hold meth_rect3? {}", meth_rect1.can_hold(meth_rect3)); //< Can rect1 hold meth_rect3? false
}

pub fn relate_fn() {
    #[derive(Debug)]struct MthRect{wid:u32,hei:u32,}
    impl MthRect{fn mth_area(self)->u32{self.wid*self.hei}}
   // 関連関数
	// implブロック内で引数に self を持たない関数も作れる
	// 構造体に関連付いているので関連関数という
	// そもそも関連関数はメソッドではなく関数
	impl MthRect {
		fn square(widyj:u32, height:u32) -> MthRect {
			MthRect { wid: widyj, hei: height }
		}
	}
	let sq = MthRect::square(16, 12);
	println!("{:?}", sq); //< MthRect { wid: 16, hei: 12 }
}

   // 複数の implブロック
	// さっきからやってますが1つの構造体に複数の implブロックを用意しても問題ないです