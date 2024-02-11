use solana_sdk::{
  pubkey::Pubkey,
  signature::write_keypair_file,
  signer::{keypair::Keypair, Signer},
};
use async_std::task;
use std::{fs::OpenOptions, io::Write};

const PREFIX: &str = "000000";

fn main() {
  for _i in 0..48 {
      task::spawn( async move {
          generate_address().await;
      });
  }

  loop {

  }
}

async fn generate_address() {
  loop {
      let keypair = &Keypair::new();
      let pubkey = keypair.pubkey();
      let prefix = &pubkey.to_string()[..PREFIX.len()];
      if prefix == PREFIX {
          print(keypair, &pubkey);
      }
  }
}

fn print(keypair: &Keypair, pubkey: &Pubkey) {
  println!(
      "pubkey: {:?}, private: {:?}",
      pubkey,
      keypair.to_base58_string()
  );
  file_operation(keypair.to_base58_string(), pubkey);
  let _result = write_keypair_file(keypair, format!("./wallet/json/{}.json", pubkey.to_string()));
}

fn file_operation(private: String, pubkey: &Pubkey) {
  let path = format!("./wallet/txt/{}.txt", pubkey.to_string());
  let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path);
  if let Ok(mut file_txt) = file {
      let _result = write!(
          &mut file_txt,
          "==========================================\np . u . b . k . e . y: {}\np . r . i . v . a . t . e: {}\n==========================================\n\n\n",
          pubkey, private
      );
  }
}