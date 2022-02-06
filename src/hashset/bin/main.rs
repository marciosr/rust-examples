#![allow(unused)]
use std::collections::HashSet;

fn vec_to_string(vec: &Vec<char>) -> String {
    let mut string = String::new();
    for i in vec {
        string.push(*i);
    }
    string
}

fn scape (a: &String, b: &String) {
	println!("{}",a.escape_unicode() );
	println!("{}",b.escape_unicode() );
	
	let c = a.escape_unicode().to_string();
	let d = b.escape_unicode().to_string();
	
	if c == d {
		println!("São iguais!");
	} else {
		println!("São diferentes!");
	}
	
	
	//assert_eq!("ΑΒγ".escape_unicode(), "ΑΒΓ".escape_unicode());

}

fn test_anagrama() {
    let entrada: &str = "ΑΒΓ"; //"diaper";
	
	println!("A entrada é ascii {}", entrada.is_ascii());

    let inputs = ["hello", "world", "zombies", "pants", "antsp", "ΑΒγ"];

    let x: HashSet<&str> = inputs
        .into_iter()
        .filter_map(|x| {
        	println!("\nA palavra testada ({}) é ascii? {}", x, x.is_ascii());
        	scape(&x.to_string(), &entrada.to_string());
            if !x.to_lowercase().eq_ignore_ascii_case(&entrada.to_lowercase()) {
                Some(x)
            } else {
                None
            }
        })
        .filter_map(|y| {
            let teste = y.to_uppercase();
            let teste2 = entrada.to_uppercase();

            let mut a: Vec<char> = y.to_uppercase().chars().collect();
            let mut b: Vec<char> = entrada.to_uppercase().chars().collect();
            a.sort_unstable();
            b.sort_unstable();

            println!("Valor de a: {:?} valor de b: {:?}", teste, teste2);

            println!(
                "É igual? {}",
                vec_to_string(&a).eq_ignore_ascii_case(&vec_to_string(&b))
            );
			
			scape(&vec_to_string(&a),&vec_to_string(&b));
            if vec_to_string(&a).eq_ignore_ascii_case(&vec_to_string(&b)) {
                Some(y)
            } else {
                None
            }
        })
        .collect();

    println!("\nTeste de iter e map: {:?}", x);

}

fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashSet<String>` in this example).
    // let mut books = HashSet::new();

    // Add some books.
    // books.insert("A Dance With Dragons".to_string());
    // books.insert("To Kill a Mockingbird".to_string());
    // books.insert("The Odyssey".to_string());
    // books.insert("The Great Gatsby".to_string());

    // Check for a specific one.
    // if !books.contains("The Winds of Winter") {
    //     println!(
    //         "We have {} books, but The Winds of Winter ain't one.",
    //         books.len()
    //     );
    // }

    // Remove a book.
    // books.remove("The Odyssey");

    // Iterate over everything.
    // for book in &books {
    //     println!("{}", book);
    // }

    // test_sort();
    
   // println!("Comparando UTF-8 {}, {}", cmp_ignore_case_utf8("ΑΒΓ", "ΑΒγ"));
    
    test_anagrama();
   // scape();
}

fn test_sort() {
    let mut v = ["x", "a", "n", "y", "r", "c", "d"];

    v.sort_unstable();
    //  let vec: Vec<char> = v.chars.collect();

    let v2 = ["a", "c", "d", "n", "r", "x", "y"];

    if v == v2 {
        println!("v e v2 são iguais!");
    }

    println!("O slice classificado é: {:?}", v);
    //println!("Transformar o slice em vetor: {:?}", vec);
}

