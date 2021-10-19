#[no_mangle]
pub extern "system" fn get_value() -> i32 {
  let mut utah_teapot = ut::UtahTeapot::new();
	utah_teapot.update(1, 1);	
	
  let aa = vec!["a", "b", "v"];
  return aa.len() as i32;
}

#[no_mangle]
pub extern fn create_instance() -> *mut ut::UtahTeapot {
  Box::into_raw(Box::new(ut::UtahTeapot::new()))
}

#[no_mangle]
pub extern fn destroy_instance(ptr: *mut ut::UtahTeapot) {
  unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern fn update(ptr: *mut ut::UtahTeapot, div: u8, sub_div: u8) {
  unsafe { (*ptr).update_clockwise(div, sub_div); }
}

#[no_mangle]
pub extern fn get_vertex_count(ptr: *const ut::UtahTeapot) -> usize {
  unsafe{ (*ptr).get_positions().len() }
}

#[no_mangle]
pub extern fn get_position(ptr: *const ut::UtahTeapot, x: &mut f32, y: &mut f32, z: &mut f32, index: usize) {
  unsafe {
     let position = (*ptr).get_positions()[index];
     *x = position.x;
     *y = position.y;
     *z = position.z;
    }
}

#[no_mangle]
pub extern fn get_normal(ptr: *const ut::UtahTeapot, x: &mut f32, y: &mut f32, z: &mut f32, index: usize) {
  unsafe {
    let normal = (*ptr).get_normals()[index];
    *x = normal.x;
    *y = normal.y;
    *z = normal.z;
  }
}

#[no_mangle]
pub extern fn get_index_count(ptr: *const ut::UtahTeapot) -> usize {
  unsafe{ (*ptr).get_indices().len() }
}

#[no_mangle]
pub extern fn get_index(ptr: *const ut::UtahTeapot, index: usize) -> u32 {
    unsafe{ (*ptr).get_indices()[index] }
}