use core::arch::asm;

macro_rules! gen_reg {
    ($name:ident => $asm_name:literal) => {
        pub struct $name;

        impl $name {
            #[inline(always)]
            pub fn read() -> u32 {
                let x;
                unsafe {
                    asm!(
                        concat!("mov {x:e}, ", $asm_name),
                        x = out(reg) x,
                        options(nomem, nostack),
                    )
                };
                x
            }

            #[inline(always)]
            pub fn write(val: u32) {
                unsafe {
                    asm!(
                        concat!("mov ", $asm_name, ", {val:e}"),
                        val = in(reg) val,
                        options(nomem, nostack),
                    );
                }
            }
        }
    }
}

macro_rules! control_reg {
    ($name:ident => $asm_name:literal) => {
        pub struct $name;

        impl $name {
            #[inline(always)]
            pub fn read() -> u64 {
                let x;
                unsafe {
                    asm!(
                        concat!("mov {x:r}, ", $asm_name),
                        x = out(reg) x,
                        options(nomem, nostack),
                    )
                };
                x
            }

            #[inline(always)]
            pub fn write(val: u64) {
                unsafe {
                    asm!(
                        concat!("mov ", $asm_name, ", {val:r}"),
                        val = in(reg) val,
                        options(nomem, nostack),
                    );
                }
            }
        }
    }
}

gen_reg!(EAX => "eax");
gen_reg!(EBX => "ebx");
gen_reg!(ECX => "ecx");

control_reg!(CR0 => "cr0");
control_reg!(CR1 => "cr1");
control_reg!(CR2 => "cr2");
control_reg!(CR3 => "cr3");
control_reg!(CR4 => "cr4");
control_reg!(CR5 => "cr5");
control_reg!(CR6 => "cr6");
control_reg!(CR7 => "cr7");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_eax() {
        EAX::write(1);
        println!("{}", EAX::read());
        EAX::write(2);
        println!("{}", EAX::read());
    }

    #[test]
    fn read_ebx() {
        println!("{}", EBX::read());
    }

    #[test]
    fn read_cr0() {
        CR0::write()
        println!("{}")
    }
}
