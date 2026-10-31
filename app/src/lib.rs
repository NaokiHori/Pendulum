//! Simulates N-body pendulum.

mod pendulum;

#[unsafe(no_mangle)]
pub extern "C" fn pendulum_new(nitems: usize) -> *mut pendulum::Pendulum {
    let pendulum = pendulum::Pendulum::new(nitems);
    Box::into_raw(Box::new(pendulum))
}

#[unsafe(no_mangle)]
/// # Safety
/// 
/// `pendulum` must be a valid, non-null pointer to a properly initialized `Pendulum` instance,
/// and must satisfy Rust's aliasing rules.
pub unsafe extern "C" fn pendulum_get_cartesian_positions(
    pendulum: *mut pendulum::Pendulum,
) -> *const f64 {
    let pendulum = unsafe { &mut *pendulum };
    pendulum.get_cartesian_positions()
}

#[unsafe(no_mangle)]
/// # Safety
/// 
/// `pendulum` must be a valid, non-null pointer to a properly initialized `Pendulum` instance,
/// and must satisfy Rust's aliasing rules.
pub unsafe extern "C" fn pendulum_integrate(pendulum: *mut pendulum::Pendulum) -> f64 {
    let pendulum = unsafe { &mut *pendulum };
    pendulum.integrate()
}

#[unsafe(no_mangle)]
/// # Safety
///
/// `pendulum` must be either null or a valid pointer previously obtained via `pendulum_new`
/// for a `Pendulum` instance.
/// 
/// This function takes ownership of the pointer and frees its memory. The caller must ensure
/// that the pointer is never used again after this call.
pub unsafe extern "C" fn pendulum_destroy(pendulum: *mut pendulum::Pendulum) {
    if !pendulum.is_null() {
        drop(unsafe { Box::from_raw(pendulum) });
    }
}
