
#![feature(asm_const)]
#![cfg_attr(not(test), no_std)]

// TODO: CFG compile errors for incompatible feature - target combos

#[cfg(target_arch = "x86")]
pub mod x86;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "powerpc")]
pub mod ppc;

pub mod common;
