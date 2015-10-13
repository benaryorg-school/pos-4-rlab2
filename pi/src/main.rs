#![feature(iter_arith)]

extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main()
{
	let pool = ThreadPool::new(3);
	let (tx,rx) = channel();

	for i in (0..100000)
	{
		let d: f64 = i as f64;
		let tx = tx.clone();
		pool.execute(move||
		{
			tx.send(f64::powi(-1.0,i)/(2.0*d+1.0)).unwrap();
		});
	}
	drop(tx);

	println!("{}",rx.iter().sum::<f64>()*4.0);
}

