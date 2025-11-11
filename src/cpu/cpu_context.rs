struct CpuContext {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,

    r8: u32,
    r8_fiq: u32,

    r9: u32,
    r9_fiq: u32,

    r10: u32,
    r10_fiq: u32,

    r11: u32,
    r11_fiq: u32,

    r12: u32,
    r12_fiq: u32,

    sp: u32,
    sp_fiq: u32,
    sp_svc: u32,
    sp_abt: u32,
    sp_irq: u32,
    sp_und: u32,

    lr: u32,
    lr_fiq: u32,
    lr_svc: u32,
    lr_abt: u32,
    lr_irq: u32,
    lr_und: u32,

    pc: u32,

    cpsr: u32,

    spsr_fiq: u32,
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_irq: u32,
    spsr_und: u32,
}
