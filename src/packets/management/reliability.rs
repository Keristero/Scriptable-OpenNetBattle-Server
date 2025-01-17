#[derive(Debug)]
pub enum Reliability {
  Unreliable,
  UnreliableSequenced,
  Reliable,
  // ReliableSequenced,
  ReliableOrdered,
}

pub fn get_reliability(reliability_byte: u8) -> Reliability {
  match reliability_byte {
    1 => Reliability::UnreliableSequenced,
    2 => Reliability::Reliable,
    // 3 => Reliability::ReliableSequenced,
    4 => Reliability::ReliableOrdered,
    _ => Reliability::Unreliable,
  }
}

pub fn get_reliability_byte(reliability: &Reliability) -> u8 {
  match reliability {
    Reliability::Unreliable => 0,
    Reliability::UnreliableSequenced => 1,
    Reliability::Reliable => 2,
    // Reliability::ReliableSequenced => 3,
    Reliability::ReliableOrdered => 4,
  }
}
