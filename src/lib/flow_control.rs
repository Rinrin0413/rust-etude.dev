pub fn bi_if() {
  // if式
    // 条件分岐が出来る
    // 例えば「もし条件が合えば、こののコードを実行し。合わないなら、このコードを実行するな」のようにできる
    // if <条件> { 処理 }
    // 合わないときの処理は  else { 処理 }
    let kejowi = 7;
    if kejowi < 5 { // 7 < 5 は成立しないので else に飛ぶ
        println!("条件は真");
    } else {
        println!("条件は偽");
    };
    // 条件部分には bool型を入れないといけない
    // ここでの kejowi < 5 は bool型を返す
    // もし条件に kejowi のみを置いた場合 整数型が条件に投げられるのでエラーとなる
    // つまり  if 0 {}  みたいなのは駄目

    // また else にも条件を付けることも可能
    // else if {}
    let kejowi2 = 6;
    if kejowi2 % 4 == 0 { // 4で割った余りが0か否か
        println!("数値は4で割り切れる");
    } else if kejowi2 % 3 == 0 { // 3で割った余りが0か否か
        println!("数値は3で割り切れる");
    } else if kejowi2 % 2 == 0 { // 2で割った余りが0か否か
        println!("数値は2で割り切れる");
    } else {
        println!("数値は4, 3, 2で割り切れない");
    }

   // let文内でif式を使う
    // if式は式なので let文に持ってくることができる
    let kedshi = if true { 5 } else { 6 };
    println!("{}", kedshi);
    // 変数は単独の型でなければならない故一連の if式内で返り値の型が全て同じでないとエラー
}