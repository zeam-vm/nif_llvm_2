#[macro_use] extern crate rustler;
// #[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

extern crate llvm_sys;

use rustler::{Env, Term, NifResult, Encoder};
use llvm_sys::core::*;
use llvm_sys::target;
use llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use llvm_sys::execution_engine::*;
use llvm_sys::LLVMModule;
use std::ffi::CString;
use std::os::raw::c_char;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.NifLlvm2",
    [("generate_code_nif", 0, generate_code_nif),
     ("execute_code_nif",  1, execute_code_nif),
     ("initialize_native_target", 0, initialize_native_target),
     ("initialize_native_asm_printer", 0, initialize_native_asm_printer)],
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

mod llvm {
    use llvm_sys::LLVMModule;
    use std::sync::RwLock;
    lazy_static! {
        pub static ref VEC_MUT: RwLock<Vec<&'static LLVMModule>> = {
            let v = Vec::new();
            RwLock::new(v)
        };
    }
}

fn write_vec_mut(module: &'static LLVMModule) -> Result<usize, String> {
    let mut v = try!(llvm::VEC_MUT.write().map_err(|e| e.to_string()));
    v.push(module);
    Ok(v.len() - 1)
}

fn read_vec(id: usize) -> Result<&'static LLVMModule, String> {
    let v = try!(llvm::VEC_MUT.read().map_err(|e| e.to_string()));
    Ok(v[id])
}

fn initialize_native_target<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
      match unsafe { target::LLVM_InitializeNativeTarget() } {
      	0 => Ok(atoms::ok().encode(env)),
      	_ => Ok(atoms::error().encode(env)),
      }
}

fn initialize_native_asm_printer<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
      match unsafe { target::LLVM_InitializeNativeAsmPrinter() } {
      	0 => Ok(atoms::ok().encode(env)),
      	_ => Ok(atoms::error().encode(env)),
      }
}


fn generate_code_nif<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    const LLVM_ERROR: i32 = 1;
    let val1 = 32;
    let val2 = 16;

    // setup our builder and module
    let builder = unsafe { LLVMCreateBuilder() };
    let mod_name = CString::new("my_module").unwrap();
    let module = unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) };

    // create our function prologue
    let function_type = unsafe {
        let mut param_types = [];
        LLVMFunctionType(LLVMInt32Type(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
    };
    let function_name = CString::new("main").unwrap();
    let function = unsafe { LLVMAddFunction(module, function_name.as_ptr(), function_type)};
    let entry_name = CString::new("entry").unwrap();
    let entry_block = unsafe { LLVMAppendBasicBlock(function, entry_name.as_ptr())};
    unsafe { LLVMPositionBuilderAtEnd(builder, entry_block); }

    // int a = 32
    let a_name = CString::new("a").unwrap();
    let a = unsafe { LLVMBuildAlloca(builder, LLVMInt32Type(), a_name.as_ptr())};
    unsafe { LLVMBuildStore(builder, LLVMConstInt(LLVMInt32Type(), val1, 0), a); }

    // int b = 16
    let b_name = CString::new("b").unwrap();
    let b = unsafe { LLVMBuildAlloca(builder, LLVMInt32Type(), b_name.as_ptr())};
    unsafe { LLVMBuildStore(builder, LLVMConstInt(LLVMInt32Type(), val2, 0), b); }

    // return a + b
    let b_val_name = CString::new("b_val").unwrap();
    let b_val = unsafe { LLVMBuildLoad(builder, b, b_val_name.as_ptr()) };
    let a_val_name = CString::new("a_val").unwrap();
    let a_val = unsafe { LLVMBuildLoad(builder, a, a_val_name.as_ptr()) };
    let ab_val_name = CString::new("ab_val").unwrap();
    unsafe {
        let res = LLVMBuildAdd(builder, a_val, b_val, ab_val_name.as_ptr());
        LLVMBuildRet(builder, res);
    }

    // verify it's all good
    let mut error: *mut c_char = 0 as *mut c_char;
    match unsafe {
        let buf: *mut *mut c_char = &mut error;
        LLVMVerifyModule(module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
    } {
        LLVM_ERROR => {
            let _err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
            // panic!("cannot verify module '{:?}.\nError: {}", mod_name, err_msg);
            Ok((atoms::error(), atoms::error()).encode(env))
        },
        _ => {
            // Clean up the builder now that we are finished using it.
            unsafe { LLVMDisposeBuilder(builder) }

            // Dump the LLVM IR to stdout so we can see what we've created
            unsafe { LLVMDumpModule(module) }

            match unsafe { write_vec_mut(&*module) } {
                Ok(r) => Ok((atoms::ok(), r).encode(env)),
                Err(_) => Ok((atoms::error(), atoms::error()).encode(env)),
            }
        },
    }

}

fn execute_code_nif<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let id: usize = try!(args[0].decode());
    match read_vec(id) {
        Err(_) => Ok((atoms::error(), atoms::error()).encode(env)),
        Ok(m) => {
            let module = m as *const LLVMModule as *mut LLVMModule;

            const LLVM_ERROR: i32 = 1;
            let val1 = 32;
            let val2 = 16;

            // create our exe engine
            let mut error: *mut c_char = 0 as *mut c_char;
            let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
            match unsafe {
                let buf: *mut *mut c_char = &mut error;
                let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
                LLVMLinkInInterpreter();
                LLVMCreateInterpreterForModule(engine_ref, module, buf)
            } {
                LLVM_ERROR => {
                    let _err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
                    // println!("Execution error: {}", err_msg);
                    unsafe { LLVMDisposeModule(module) }
                    Ok((atoms::error(), atoms::error()).encode(env))
                },
                _ => {
                    // run the function!
                    let func_name = CString::new("main").unwrap();
                    let named_function = unsafe { LLVMGetNamedFunction(module, func_name.as_ptr()) };
                    let mut params = [];
                    let func_result = unsafe { LLVMRunFunction(engine, named_function, params.len() as u32, params.as_mut_ptr()) };
                    let result = unsafe { LLVMGenericValueToInt(func_result, 0) };
                    println!("{} + {} = {}", val1, val2, result);
                    // Clean up the module after we're done with it.
                    unsafe { LLVMDisposeModule(module) }
                    Ok(atoms::ok().encode(env))
                }
            }

        },
    }

}
