extern crate primeiter;

use primeiter::PrimeIter;

use std::env::args;

fn main()
{
	let args: Vec<_> = args().collect();
	assert_eq!(args.len(),3);
	let low = args[1].parse().unwrap();
	let high = args[2].parse().unwrap();

	for p in PrimeIter::new().skip_while(|i|*i<low).take_while(|i|*i<=high)
	{
		println!("{} is prime",p);
	}
}

