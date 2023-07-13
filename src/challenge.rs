use rand::Rng;
use sha2::{Digest, Sha256};

pub const CHALLENGE_SIZE: usize = 16;
pub const SOLUTION_SIZE: usize = 16;
pub const COMPLEXITY: usize = 5;

#[derive(Debug)]
pub struct Challenge {
    pub challenge: [u8; CHALLENGE_SIZE],
    pub hash: Sha256,
}

impl Default for Challenge {
    fn default() -> Self {
        Self::new_rand()
    }
}

impl Challenge {
    pub fn new(challenge: [u8; CHALLENGE_SIZE]) -> Self {
        Self {
            challenge,
            hash: hash(challenge),
        }
    }

    pub fn new_rand() -> Self {
        let challenge = rand::thread_rng().gen::<[u8; CHALLENGE_SIZE]>();

        Self {
            challenge,
            hash: hash(challenge),
        }
    }

    pub fn solve(&self) -> ([u8; SOLUTION_SIZE], u128) {
        // Iterate random values - PoW
        let mut rng = rand::thread_rng();
        let mut tries: u128 = 0;
        loop {
            let solution: [u8; SOLUTION_SIZE] = rng.gen();
            tries += 1;
            if self.check_solution(&solution) {
                return (solution, tries);
            }
        }
    }

    pub fn check_solution(&self, solution: &[u8; SOLUTION_SIZE]) -> bool {
        // Check wether the newly generated hash passes the complexity
        let mut hash = self.hash.clone();
        hash.update(solution);
        let hash = hash.finalize();

        let mut leading_zeros = 0;

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

// Helper functions

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
