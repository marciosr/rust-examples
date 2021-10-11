#![allow(unused)]
use std::ptr;
use std::mem;


fn main() {
	//vec_parts();
	vec_retain();
	vec_leak ();
	vec_split_off ();
}

fn vec_parts () {
	let v = vec![1, 2, 3, 4, 5];
	// Prevent running `v`'s destructor so we are in complete control
	// of the allocation.
	let mut v = mem::ManuallyDrop::new(v);

	// Pull out the various important pieces of information about `v`
	let p = v.as_mut_ptr();
	let len = v.len();
	let cap = v.capacity();

	println!("Vetor antes: {:?}", v);
	println!("Ponteiro: {:?}\nTamanho: {}\nCapacidade: {}\n", p, len, cap);
	unsafe {
		  // Overwrite memory with 4, 5, 6
		  for i in 0..len as isize {
		      ptr::write(p.offset(i), 6 + i);
		  }

		  // Put everything back together into a Vec
		  let rebuilt = Vec::from_raw_parts(p, len, cap);
		  assert_eq!(rebuilt, [6, 7, 8, 9, 10]);
		  println!("Vetor depois: {:?}", rebuilt);
		  println!("Ponteiro depois: {:?}", rebuilt.as_ptr());
		  println!("Tamanho depois: {:?}", rebuilt.len());
		  println!("p depois: {:?}", p);
	}
}

fn vec_retain () {
	let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
	vec.retain(|&x| x % 2 == 0);
	println!("\nVetor orgininal: [1, 2, 3, 4, 5, 6, 7]\nResultado do método vec.retain(): {:?}", vec);
}


fn vec_leak () {
	let x = vec![1, 2, 3];
	let static_ref: &'static mut [usize] = x.leak();
	
	static_ref[0] += 1;
	
	println!("\nResultado do método vec.leak(): {:?}", static_ref);
}

fn vec_split_off () {
	let mut vec = vec![1, 2, 3, 4, 5];
	let vec2 = vec.split_off(2);
	
	println!("\nResultado do vec.split_off()\nVetor original: {:?}\nVetor receptor: {:?}", vec, vec2);
}
