#[macro_export] macro_rules! uiua {
    ($code:expr) => {{
        let mut comp = Compiler::new();
        let asm = comp.load_str(&format!($code)).unwrap().finish();
        let mut uiua = Uiua::with_native_sys();
        uiua.run_asm(asm).unwrap();
        uiua
    }}
}