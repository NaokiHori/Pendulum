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
/// Assumed to be used properly
pub unsafe extern "C" fn pendulum_get_cartesian_positions(
    pendulum: *mut pendulum::Pendulum,
) -> *const f64 {
    let pendulum = unsafe { &mut *pendulum };
    pendulum.get_cartesian_positions()
}

#[unsafe(no_mangle)]
/// # Safety
///
/// Assumed to be used properly
pub unsafe extern "C" fn pendulum_integrate(pendulum: *mut pendulum::Pendulum) -> f64 {
    let pendulum = unsafe { &mut *pendulum };
    pendulum.integrate()
}

#[unsafe(no_mangle)]
/// # Safety
///
/// Assumed to be used properly
pub unsafe extern "C" fn pendulum_destroy(pendulum: *mut pendulum::Pendulum) {
    if !pendulum.is_null() {
        drop(unsafe { Box::from_raw(pendulum) });
    }
}
