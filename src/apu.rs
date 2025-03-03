use super::cpu::*;

#[derive(Copy, Clone)]
pub enum PulseDutyCycle {
    Duty12_5,
    Duty25_0,
    Duty50_0,
    Duty75_0,
}

#[derive(Copy, Clone)]
pub struct PulseSound {
    // Initalize Duty Cycle $4000 - $4004
    pub duty_cycle: PulseDutyCycle,
    // Playback time counter enabled
    pub is_length_counter_halt: bool,
    // sound selection
    pub is_constant_volume: bool,
    // volum 4bit
    pub volume: u8,

    // frequency sweep $4001 / $4005
    pub is_sweep_enable: bool,
    // value
    pub sweep_period: u8,
    // direction (add/sub)
    pub is_sweep_negative: bool,
    // range
    pub sweep_shift: u8,

    // $4002, $4003 / $4006, $4007
    // Frequency   
    pub timer_value: u16,
    // playback
    pub length_counter_load: u8,
}
impl Default for PulseSound {
    fn default() -> Self {
        Self {
            duty_cycle: PulseDutyCycle::Duty12_5,
            is_length_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            is_sweep_enable: false,
            sweep_period: 0,
            is_sweep_negative: false,
            sweep_shift: 0,
            timer_value: 0,
            length_counter_load: 0,
        }
    }
}

impl PulseSound {
    pub fn get_freq(&self) -> u32 {
        CPU_FREQ / (16 * (u32::from(self.timer_value) + 1))
    }
}

#[derive(Copy, Clone)]
pub struct TriangleSound {
    // $4008
    /// Playback time counter enabled
    pub is_length_counter_halt: bool,
    // Playback time counter value
    pub counter_load: u8,
    // frequency $400a, $400b
    pub timer_value: u16,
    // playback time
    pub length_counter_load: u8,
}

impl Default for TriangleSound {
    fn default() -> Self {
        Self {
            is_length_counter_halt: false,
            counter_load: 0,
            timer_value: 0,
            length_counter_load: 0,

        }
    }
}

#[derive(Copy, Clone)]
pub struct NoiseSound {
    // Playback time counter enabled $400c
    pub is_length_counter_halt: bool,
    // Acoustic selection
    pub is_constant_volume: bool,
    // 4bit volume val
    pub volume: u8,
    // $400E
    pub is_noise_type_loop: bool,
    // Playback time counter value
    pub noise_period: u8,
    // $400f
    pub length_counter_load: u8,
}

impl Default for NoiseSound {
    fn default() -> Self {
        Self {
            is_length_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            is_noise_type_loop: false,
            noise_period: 0,
            length_counter_load: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DmcSound {
    / / $ 4010/ / / Rūpu-ji ni warikomi yūkō pub is _ irq _ inēburu: Bool, / / / rūpu yūkō pub is _ loop _ inēburu: Bool, / / / sanpururētobitto 4 bitto pub frequency: U 8, / / $ 4011/ / / saisei jikan pub load _ kauntā: U 8, / / $ 4012/ / / yomikomi-saki adoresu/ / / $ C 000 - FFFF o sanshō surunode 11 AAAAAA - AA 000000 pub sanpuru _ addr: U 8, / / $ 4013/ / / rūpu ni tsukau dēta-ryō/ / / 0000 LLLL, LLLL 0001 pub sanpuru _ rengusu: U 8,
Show more
387 / 5000
Translation results
// $ 4010
     /// Interrupt enabled during Loop
     pub is_irq_enable: bool,
     /// Loop enabled
     pub is_loop_enable: bool,
     /// Sample rate bit 4bit
     pub frequency: u8,
     // $ 4011
     /// Playback time
     pub load_counter: u8,
     // $ 4012
     /// Read-to address
     /// Refers to $ C000-FFFF, so 11AAAAAA-AA000000
     pub sample_addr: u8,
     // $ 4013
     /// Amount of data used for the loop
     /// 0000LLLL, LLLL0001
     pub sample_length: u8, 
}

impl Default for DmcSound {
    fn default() -> Self {
        Self {
            is_irq_enable: false,
            is_loop_enable: false,
            frequency: 0,
            load_counter: 0,
            sample_addr: 0,
            sample_length: 0,
        }
    }
}

#[derive(Clone)]
pub struct Apu {
    // Addition 11bit linked to CPU cycle
    pub frame_seq_counter: u16,
}
impl Default for Apu {
    fn default() -> Self {
        Self {
            frame_seq_counter: 0,
        }
    }
}

impl Apu {
    // FrameSeq
    // TODO: Deadcode
    #[allow(dead_code)]
    fn increment_seq(&mut self, cpu_cyc: u8) {
        self.frame_seq_counter = (self.frame_seq_counter + u16::from(cpu_cyc)) & 0x03ff;
        // 11bit
    }
    // APU
    pub fn step(&mut self, _cpu: &mut Cpu, _cpu_cyc: u8) {
        // TODO: kms
    }
}
