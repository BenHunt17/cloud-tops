pub enum CpuMode {
    User = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl CpuContext {
    fn is_bit_set(&self, bit: u8) -> bool {
        ((self.cpsr >> bit) & 1) == 1
    }

    fn set_bit(&mut self, bit: u8, val: bool) {
        if val {
            self.cpsr |= (1 << bit)
        } else {
            self.cpsr &= !(1 << bit)
        }
    }

    //Condition flags

    pub fn n(&self) -> bool {
        self.is_bit_set(31)
    }
    pub fn z(&self) -> bool {
        self.is_bit_set(30)
    }
    pub fn c(&self) -> bool {
        self.is_bit_set(29)
    }
    pub fn v(&self) -> bool {
        self.is_bit_set(28)
    }

    pub fn set_n(&mut self, val: bool) {
        self.set_bit(31, val)
    }
    pub fn set_z(&mut self, val: bool) {
        self.set_bit(30, val)
    }
    pub fn set_c(&mut self, val: bool) {
        self.set_bit(29, val)
    }
    pub fn set_v(&mut self, val: bool) {
        self.set_bit(28, val)
    }

    // IRQ disable
    pub fn irq_disable(&self) -> bool {
        self.is_bit_set(7)
    }

    pub fn set_irq_disable(&mut self, val: bool) {
        self.set_bit(7, val)
    }

    //FIQ disable
    pub fn fiq_disable(&self) -> bool {
        self.is_bit_set(6)
    }

    pub fn set_fiq_disable(&mut self, val: bool) {
        self.set_bit(6, val)
    }

    //THUMB state
    fn is_thumb_state(&self) -> bool {
        self.is_bit_set(5)
    }

    pub fn set_thumb_state(&mut self, val: bool) {
        self.set_bit(5, val)
    }

    // CPU mode

    fn get_cpu_mode(&self) -> CpuMode {
        match (self.cpsr & 0b11111) {
            0b10000 => CpuMode::User,
            0b10001 => CpuMode::Fiq,
            0b10010 => CpuMode::Irq,
            0b10011 => CpuMode::Supervisor,
            0b10111 => CpuMode::Abort,
            0b11011 => CpuMode::Undefined,
            0b11111 => CpuMode::System,
            _ => unreachable!("Invalid CPU mode bits in CPSR"),
        }
    }

    pub fn set_cpu_mode(&mut self, mode: CpuMode) {
        self.cpsr = (self.cpsr & !0b11111) | (mode as u32)
    }
}
