use std::ops::{Index, Range};
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
//use std::rc::Rc;

extern crate num_cpus;

fn is_prime(buckets: &Vec<Arc<Vec<u64>>>, target: u64) -> bool {
	let upper_value = target as f64;
	let upper_value = upper_value.sqrt().floor() as u64;

	for bucket in buckets {
		for prime in &**bucket {
			if *prime > upper_value {
				//println!("{} > {} -- is prime", *prime, upper_value);
				return true;
			}
			else {
				//println!("{}%{} == {}", target, *prime, target % *prime );
				if (target % *prime) == 0 {
					//println!("{} not prime!", target);
					return false;
				}
			}
		}
	}

	return true;
}

#[derive(Default)]
pub struct PrimeGenerator {
	primes: Vec<u64>,
}

impl PrimeGenerator {

	fn compute_primes(&mut self, count: usize) {
		// With the exception of 2, alll primes are odd.
		// We will start with 2 in our list,
		// and then we only need to cheeck odd numbers.
		if count > 0 {
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

	fn compute_primes_parallel(&mut self, count: usize) {

		let thread_limit = num_cpus::get()/2;
		println!("Using {} threads", thread_limit);

		if count > 0 {
			self.primes = vec![2,3];

			let mut receivers: VecDeque<Receiver<Vec<u64>>> = VecDeque::new();
			let mut buckets: Vec<Arc<Vec<u64>>> = vec![Arc::new(self.primes.clone())];

			let mut next_bucket: u64 = 3;

			while self.primes.len() < count {
			//{
				// First we will spin up as many threads to make as many buckets as we can.
				// Then we will wait for the thread making the lowest bucket to finish.
				// With this new bucket, we will add its values to the original vector.
				// On the next loop. We will repeat this.
				//
				// Each loop will only receive the values of one bucket,
				// but may spin up many threads.
				//
				// Each bucket will span a range of [(n-1)^2, n^2)
		
				// Make a list of all of the current buckets
				// This gets passed into our threads
				let mut current_buckets: Vec<Arc<Vec<u64>>> = Vec::new();
				for bucket in &buckets {
					current_buckets.push(Arc::clone(&bucket));
				}
				let current_buckets = Arc::new(current_buckets); // We're done building this

				let biggest_prime = self.primes[self.primes.len()-1];
				while biggest_prime >= next_bucket-1 && receivers.len() < thread_limit {
				//{
					let (tx, rx) = mpsc::channel::<Vec<u64>>();

					let lower_bound = next_bucket-1;
					let lower_bound = lower_bound*lower_bound;

					// For the thread to work correctly, the starting boundary must be odd.
					// If we are on an even number, we can skip past it.
					let lower_bound = if lower_bound%2 == 0 { lower_bound+1 } else { lower_bound };
					let upper_bound = next_bucket*next_bucket;

					let current_buckets = Arc::clone(&current_buckets);

					//println!("added bucket {} - {:?}", next_bucket, (lower_bound..upper_bound));

					thread::spawn(move || {
						//println!("{:?}", (lower_bound..upper_bound));

						let mut bucket: Vec<u64> = Vec::new();

						for i in (lower_bound..upper_bound).step_by(2) {
							if is_prime(&*current_buckets, i) {
								bucket.push(i);
							}
						}

						let _ = tx.send(bucket); // we do not care if this fails
					});

					receivers.push_back(rx);
					next_bucket += 1;
				}

				let new_bucket = Arc::new(receivers.pop_front().unwrap().recv().unwrap());
				for value in &*new_bucket {
					self.primes.push(*value);
				}
				//println!("Computed {} primes", self.primes.len());
				buckets.push(new_bucket);

				
			}
		}
	}

    /// Gets a new prime generator which will contain primes from the range of [0, count)
	pub fn new_single(count: usize) -> PrimeGenerator {
		let mut gen = PrimeGenerator {
			primes: Vec::new(),
		};
        gen.compute_primes(count);
        return gen;
	}

	pub fn new_parallel(count: usize) -> PrimeGenerator {
		let mut gen = PrimeGenerator {
			primes: Vec::new(),
		};
        gen.compute_primes_parallel(count);
        return gen;
	}

	pub fn new(count: usize) -> PrimeGenerator {
		PrimeGenerator::new_parallel(count)
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
