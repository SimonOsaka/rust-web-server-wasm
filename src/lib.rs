#[no_mangle]
pub extern "C" fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod test {
    use wasmer::{imports, Instance, Module, Store, Value};

    #[test]
    pub fn test() {
        let base_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("wasm32-unknown-unknown");
        println!("{:?}", &base_path);

        let store = Store::default();
        let module = Module::from_file(&store, base_path.join("release/rust_web_server_wasm.wasm"))
            .expect("module wasm error");
        let import_object = imports! {};
        let instance = Instance::new(&module, &import_object).expect("instance error");
        let sum = instance
            .exports
            .get_function("sum")
            .expect("get function err");
        let args = [Value::I32(88), Value::I32(2)];
        let result = sum.call(&args).expect("call err");
        println!("result: {:?}", result);
        //
        let sum_native = sum.native::<(i32, i32), i32>().expect("native err");
        let result = sum_native.call(3, 4).expect("native call err");
        println!("result: {:?}", result);
    }
}
