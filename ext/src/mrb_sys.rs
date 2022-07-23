#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("../generated/mruby.rs");

// hand implementation
/**
 * Function requires n arguments.
 *
 * @param n
 *      The number of required arguments.
 */
#[inline]
pub fn MRB_ARGS_REQ(n: u32) -> mrb_aspec {
    (((n) & 0x1f) << 18)
}

/**
 * Function takes n optional arguments
 *
 * @param n
 *      The number of optional arguments.
 */
#[inline]
pub fn MRB_ARGS_OPT(n: u32) -> mrb_aspec {
    (((n) & 0x1f) << 13)
}

/**
 * Function takes n1 mandatory arguments and n2 optional arguments
 *
 * @param n1
 *      The number of required arguments.
 * @param n2
 *      The number of optional arguments.
 */
#[inline]
pub fn MRB_ARGS_ARG(n1: u32, n2: u32) -> mrb_aspec {
    (MRB_ARGS_REQ(n1) | MRB_ARGS_OPT(n2))
}

/** rest argument */
#[inline]
pub fn MRB_ARGS_REST() -> mrb_aspec {
    (1 << 12)
}

/** required arguments after rest */
#[inline]
pub fn MRB_ARGS_POST(n: u32) -> mrb_aspec {
    (((n) & 0x1f) << 7)
}

/** keyword arguments (n of keys, kdict) */
#[inline]
pub fn MRB_ARGS_KEY(n1: u32, n2: u32) -> mrb_aspec {
    ((((n1) & 0x1f) << 2) | (if (n2 != 0) { (1 << 1) } else { 0 }))
}

/**
 * Function takes a block argument
 */
#[inline]
pub fn MRB_ARGS_BLOCK() -> mrb_aspec {
    1
}

/**
 * Function accepts any number of arguments
 */
#[inline]
pub fn MRB_ARGS_ANY() -> mrb_aspec {
    MRB_ARGS_REST()
}

/**
 * Function accepts no arguments
 */
#[inline]
pub fn MRB_ARGS_NONE() -> mrb_aspec {
    (0)
}

#[inline]
pub unsafe fn mrb_gc_arena_save(mrb: *const mrb_state) -> i32 {
    (*mrb).gc.arena_idx
}

#[inline]
pub unsafe fn mrb_gc_arena_restore(mrb: *mut mrb_state, idx: i32) {
    (*mrb).gc.arena_idx = idx;
}
