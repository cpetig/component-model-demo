pub trait Markdown {
  fn render(input: String,) -> String;
}

#[doc(hidden)]
pub unsafe fn call_render<T: Markdown>(arg0: i32,arg1: i32,) -> i32 {
  let len0 = arg1 as usize;
  let result1 = T::render(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap());
  let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
  let vec3 = (result1.into_bytes()).into_boxed_slice();
  let ptr3 = vec3.as_ptr() as i32;
  let len3 = vec3.len() as i32;
  core::mem::forget(vec3);
  *((ptr2 + 4) as *mut i32) = len3;
  *((ptr2 + 0) as *mut i32) = ptr3;
  ptr2
}

#[doc(hidden)]
pub unsafe fn post_return_render<T: Markdown>(arg0: i32,) {
  wit_bindgen_guest_rust::rt::dealloc(*((arg0 + 0) as *const i32), (*((arg0 + 4) as *const i32)) as usize, 1);
}

#[repr(align(4))]
struct _RetArea([u8; 8]);
static mut _RET_AREA: _RetArea = _RetArea([0; 8]);

/// Declares the export of the component's world for the
/// given type.

macro_rules! export_markdown(($t:ident) => {
  const _: () = {
    
    #[doc(hidden)]
    #[export_name = "render"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_markdown_render(arg0: i32,arg1: i32,) -> i32 {
      call_render::<$t>(arg0,arg1,)
    }
    
    #[doc(hidden)]
    #[export_name = "cabi_post_render"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_markdown_render(arg0: i32,) {
      markdown::post_return_render::<$t>(arg0,)
    }
    
  };
  
  #[used]
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  static __FORCE_SECTION_REF: fn() = __force_section_ref;
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  fn __force_section_ref() {
    __link_section()
  }
});

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:markdown"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 131] = [2, 0, 8, 109, 97, 114, 107, 100, 111, 119, 110, 8, 109, 97, 114, 107, 100, 111, 119, 110, 8, 109, 97, 114, 107, 100, 111, 119, 110, 0, 97, 115, 109, 11, 0, 1, 0, 7, 64, 1, 65, 2, 1, 65, 2, 1, 64, 1, 5, 105, 110, 112, 117, 116, 115, 0, 115, 4, 6, 114, 101, 110, 100, 101, 114, 0, 1, 0, 4, 8, 109, 97, 114, 107, 100, 111, 119, 110, 22, 112, 107, 103, 58, 47, 109, 97, 114, 107, 100, 111, 119, 110, 47, 109, 97, 114, 107, 100, 111, 119, 110, 4, 0, 11, 26, 1, 8, 109, 97, 114, 107, 100, 111, 119, 110, 13, 112, 107, 103, 58, 47, 109, 97, 114, 107, 100, 111, 119, 110, 3, 0];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
