use rand::Rng;

pub fn gen_ran() -> u8 {
    // 乱数の生成器作成
    let mut rng = rand::thread_rng();
    // 乱数生成
    let n: u8 = rng.gen();
    n
}