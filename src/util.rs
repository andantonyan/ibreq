use rand::Rng;

pub fn gen_random_byte() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen::<u8>()
}
