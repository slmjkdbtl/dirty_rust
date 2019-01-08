// wengwengweng

use dirty::*;

fn main() {

	ketos::run_code(r#"
		(define (foo a)
			(* a 2))
		(println "yo")
	"#);

}

