#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod calculator {
        #[allow(dead_code, clippy::all)]
        pub mod calculate {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum Op {
                Add,
                Sub,
            }
            impl ::core::fmt::Debug for Op {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Op::Add => f.debug_tuple("Op::Add").finish(),
                        Op::Sub => f.debug_tuple("Op::Sub").finish(),
                    }
                }
            }
            impl Op {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Op {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Op::Add,
                        1 => Op::Sub,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn calculate(op: Op, x: u32, y: u32) -> u32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:calculator/calculate@0.1.0")]
                    extern "C" {
                        #[link_name = "calculate"]
                        fn wit_import(_: i32, _: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        op.clone() as i32,
                        _rt::as_i32(&x),
                        _rt::as_i32(&y),
                    );
                    ret as u32
                }
            }
        }
    }
}
mod _rt {
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:app:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 247] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07~\x01A\x02\x01A\x02\x01\
B\x04\x01m\x02\x03add\x03sub\x04\0\x02op\x03\0\0\x01@\x03\x02op\x01\x01xy\x01yy\0\
y\x04\0\x09calculate\x01\x02\x03\x01$component:calculator/calculate@0.1.0\x05\0\x04\
\x01\x17component:app/app@0.1.0\x04\0\x0b\x09\x01\0\x03app\x03\0\0\0G\x09produce\
rs\x01\x0cprocessed-by\x02\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060.\
30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
