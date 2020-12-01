use std::ops::{Index, Range};

pub struct PrimeGenerator {
	primes: Vec<u64>,
}

impl PrimeGenerator {

	fn compute_primes(&mut self, count: usize) {
		// With the exception of 2, alll primes are odd.
		// We will start with 2 in our list,
		// and then we only need to cheeck odd numbers.
		if count > 0 {
			// TODO Don't append extra 2
			self.primes.push(2);

			let mut number: u64 = 3;

			while self.primes.len() < count {
				if self.is_prime(number) {
					self.primes.push(number);
				}

				number += 2;
			}
		}
	}

    /// Gets a new prime generator which will contain primes from the range of [0, count)
	pub fn new(count: usize) -> PrimeGenerator {
		let mut gen = PrimeGenerator {
			primes: Vec::new(),
		};
        gen.compute_primes(count);
        return gen;
	}

    /// Gets a slice of all of the calculated primes
	pub fn get_primes(&self) -> &[u64] {
		&self.primes[..]
	}

    /// Gets the total number of primes
    pub fn count(&self) -> usize {
        self.primes.len()
    }

    /// Gets the total number of primes
    pub fn len(&self) -> usize {
        self.primes.len()
    }

    /// Gets the nth prime, where n must be in the range of [0, total_primes)
    pub fn get_prime(&self, n: usize) -> u64 {
        self.primes[n]
    }

    /// Checks if `target` is a prime. Returns true if is, false otherwise.
    /// If the number falls outside of the allowed range, false is always returned.
	pub fn is_prime(&self, target: u64) -> bool {
		let upper_value = target as f64;
		let upper_value = upper_value.sqrt().floor() as u64;

		for prime in &self.primes {
			if *prime > upper_value {
				return true;
			}
			if (target % *prime) == 0 {
				return false;
			}
		}

		return true;
	}
}

impl Index<usize> for PrimeGenerator {
    type Output = u64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.primes[i]
    }
}

impl Index<Range<usize>> for PrimeGenerator {
    type Output = [u64];

    fn index(&self, r: Range<usize>) -> &Self::Output {
        &self.primes[r]
    }
}
