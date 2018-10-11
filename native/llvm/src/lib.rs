#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

extern crate llvm_sys;

use rustler::{Env, Term, NifResult, Encoder};


mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.NifLlvm2",
    [("generate_code", 1, generate_code)],
    None
}

/*

int main() {
  int a = 32;
  int b = 16;
  return a + b;
}

define i32 @main() #0 {
  %1 = alloca i32, align 4
  %a = alloca i32, align 4
  %b = alloca i32, align 4
  store i32 0, i32* %1
  store i32 32, i32* %a, align 4
  store i32 16, i32* %b, align 4
  %2 = load i32, i32* %a, align 4
  %3 = load i32, i32* %b, align 4
  %4 = add nsw i32 %2, %3
  ret i32 %4
}

*/

fn generate_code<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
//    let num1: i64 = try!(args[0].decode());

    Ok(atoms::ok().encode(env))
}
