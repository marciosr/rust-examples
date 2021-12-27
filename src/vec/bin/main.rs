#![allow(unused)]
use std::ptr;
use std::mem;


fn main() {
	//vec_parts();
	vec_retain();
	vec_leak ();
	vec_split_off ();
	vec_pop();
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
	println!("\n\n  Exemplo de uso do método vec.retain()
	Vetor orgininal: [1, 2, 3, 4, 5, 6, 7]
	vec.retain(|&x| x % 2)
	Método vec.retain() filtrando apenas os números divisíveis por 2.
	Resultado	: {:?}", vec);
}

// Consome um vetor e retorna uma referência mutável para
// seu conteúdo.
// No exemplo abaixo um vetor imutável pôde ser alterado
// apartir do retorno do método .leak().
// O vetor original foi consumido, portanto não pode ser mais utilizado.
fn vec_leak () {
	let x = vec![1, 2, 3];
	let static_ref: &'static mut [usize] = x.leak();
	
	static_ref[0] += 20;

	println!("\n\n  Exemplo de uso do método vec.leak()
	Vetor original [1, 2, 3]
	Aplicando static_ref[0] += 1;
	Resultado do método vec.leak(): {:?}", static_ref);
}

// Divide um vetor removendo seu conteúdo a partir de uma posição informada e
// atribuindo o restante a um novo vetor.
fn vec_split_off () {
	let mut vec = vec![12, 45, 88, 19, 13];
	let vec2 = vec.split_off(2);
	
	println!("\n\n  Exemplo de uso do método vec.split_off()
	Vetro original: [1, 2, 3, 4, 5]
	Resultado do vec.split_off() a partir da posição 2
	Vetor original: {:?}
	Vetorreceptor: {:?}", vec, vec2);
}


// Método .pop()
// Retorna o último elemento de um vetor e o retorna, ou retorna None se estiver vazio
// Retorna Some(T) ou None.
fn vec_pop () {
	let mut vec = vec![12, 45, 88];

	let a = vec.pop();
	let b = vec.pop();
	let c = vec.pop();
	let d = vec.pop();

	println!("\n\n  Método pop().
	Vetor original = [12, 45, 88]
	Resultado da primeira aplicação: {:?}
	Resultado da segunda aplicação: {:?}
	Resultado da terceira aplicação: {:?}
	Resultado da quarta aplicação: {:?}", a.unwrap(), b, c, d);

}
