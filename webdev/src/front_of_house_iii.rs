// front_of_house_iiiモジュールの中身をそのまま置くことができる
pub mod hosting_iii {
    pub fn add_to_waitlist_iii() {}
}
 
// 分割されたファイル先でも更に分割することが可能
pub mod hosting_iv; // 同階層の hosting_iv.rs を読み込む

//✔試しに ./hosting_iv.rs 参照