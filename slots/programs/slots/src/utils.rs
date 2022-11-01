use anchor_lang::{
  prelude::*, 
  solana_program::{
    clock,
    keccak::{hash, Hash}
  }
};
use crate::constants::*;

pub fn get_random() -> u32 {
  let slot = clock::Clock::get().unwrap().unix_timestamp as u64;
  let hash = hash(&slot.to_be_bytes());
  let buf: [u8; 32] = Hash::to_bytes(hash);
  let slice: [u8; 4] = [buf[10], buf[12], buf[8], buf[16]];
  u32::from_be_bytes(slice)  
}

pub fn get_status(bet_no: u8, win_percents: [[u16; 3]; 6], jackpot: u64, lose: bool) -> (u32, u64) {
  let mut rand = get_random();
  let price = BET_PRICES[bet_no as usize];
  
  let mut max = rand % 2 + 1;
  rand = rand % 10000;
  for i in 0..3 {
    let bn = bet_no as usize;
    let mut low: u32 = 0;
    if i < 2 {
      low = win_percents[bn][i + 1].into();
    }
    if rand >= low && rand < win_percents[bn][i].into() && lose == false {
      max = 3 + i as u32;
    }
  }

  let multipler = (max - 1) * 10 - rand % 10;

  let mut earned = 0;
  if max >= 3 {
    earned = price.checked_mul(multipler as u64).unwrap().checked_div(10).unwrap();
    if max == 5 && jackpot > 0 && bet_no > 3 {
      earned = jackpot;
    }
  }

  msg!("Status: {:?}", rand);
  msg!("Max Equal: {:?}", max);

  return (rand, earned);
}