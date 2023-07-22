use rand::Rng;
use sha2::{Digest, Sha256};

pub const CHALLENGE_SIZE: usize = 16;
pub const SOLUTION_SIZE: usize = 16;
pub const COMPLEXITY: usize = 4;

#[derive(Debug)]
pub struct Challenge {
    pub challenge: [u8; CHALLENGE_SIZE], // Challenge bytes
    pub hash: Sha256, // A precalculated hash that is used for both for solving and checking the solution on both server and client sides
}

// Default implementation for the Challenge object
impl Default for Challenge {
    fn default() -> Self {
        Self::new_rand()
    }
}

impl Challenge {
    // Creates the challenge object with a given challenge, used at the client side
    pub fn new(challenge: [u8; CHALLENGE_SIZE]) -> Self {
        Self {
            challenge,
            hash: hash(challenge),
        }
    }

    // Creates the challenge object with random string, used at the initiators server side
    pub fn new_rand() -> Self {
        let challenge = rand::thread_rng().gen::<[u8; CHALLENGE_SIZE]>();

        Self {
            challenge,
            hash: hash(challenge),
        }
    }

    // Solve the challenge. The actual PoW.
    pub fn solve(&self) -> ([u8; SOLUTION_SIZE], u128) {
        // Iterate random values - PoW
        let mut rng = rand::thread_rng();
        let mut tries: u128 = 0;
        // Loop through tandom solutions till finding the correct one
        loop {
            let solution: [u8; SOLUTION_SIZE] = rng.gen();
            tries += 1;
            if self.check_solution(&solution) {
                return (solution, tries);
            }
        }
    }

    // Check the solution versus the challenge
    pub fn check_solution(&self, solution: &[u8; SOLUTION_SIZE]) -> bool {
        // Check wether the newly generated hash passes the complexity
        let mut hash = self.hash.clone();
        hash.update(solution);
        let hash = hash.finalize();

        let mut leading_zeros = 0;

        // Hexadecimal check 
        // 0000a718d067546a563908f32feef858f03ccfff4ce16b77e172287ac53fb3ee - passes with complexity 4
        // only 2 bytes are 0x00
        for c in hash.iter().take(COMPLEXITY / 2 + 1) {
            if c >> 4 == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
            if c & 0xF == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        leading_zeros >= COMPLEXITY
    }
}

// Creates the first precompiled challenge hash
fn hash(challenge: [u8; CHALLENGE_SIZE]) -> Sha256 {
    // Hash the challenge
    let mut hash = Sha256::new();
    hash.update(challenge);
    hash
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_valid_solution() {
        let challenge = Challenge::new_rand();
        assert!(!challenge.check_solution(&[0u8; SOLUTION_SIZE]));
    }

    #[test]
    fn test_puzzle_solve() {
        let challenge = Challenge::new_rand();
        assert!(!challenge.check_solution(&[0u8; SOLUTION_SIZE]));
        let (solution, tries) = challenge.solve();
        assert!(tries > 0);
        assert!(challenge.check_solution(&solution));

        let mut hash = Sha256::default();
        hash.update(challenge.challenge);
        hash.update(solution);
        let hash = format!("{:x}", hash.finalize());
        assert!(hash.starts_with(&"0".repeat(COMPLEXITY)));
    }
}
