
                pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073815552,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v2",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "adc1en",
                    },
                ),
                reset: None,
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073816320,
        registers: Some(
            PeripheralRegisters {
                kind: "adccommon",
                version: "v2",
                block: "ADC_COMMON",
                ir: &adccommon::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v1",
                block: "CRC",
                ir: &crc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "crcen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "crcrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 1073771520,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v2",
                block: "DAC",
                ir: &dac::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "dacen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "dacrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "OUT1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM6_DAC",
            },
        ],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "f4",
                block: "DBGMCU",
                ir: &dbgmcu::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 1073897472,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v2",
                block: "DMA",
                ir: &dma::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "dma1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "dma1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "DMA1_STREAM0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_STREAM1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_STREAM2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_STREAM3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_STREAM4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_STREAM5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_STREAM6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_STREAM7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073898496,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v2",
                block: "DMA",
                ir: &dma::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "dma2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "dma2rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "DMA2_STREAM0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_STREAM1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_STREAM2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_STREAM3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_STREAM4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_STREAM5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_STREAM6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_STREAM7",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 1073822720,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "v1",
                block: "EXTI",
                ir: &exti::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9_5",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073888256,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "f4",
                block: "FLASH",
                ir: &flash::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "FLASH",
            },
        ],
    },
    Peripheral {
        name: "FMPI2C1",
        address: 1073766400,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "fmpi2c1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "fmpi2c1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "dckcfgr2",
                        field: "fmpi2c1sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "FMPI2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "FMPI2C1_EV",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 1073872896,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "gpioaen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "gpioarst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1073873920,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "gpioben",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "gpiobrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1073874944,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "gpiocen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "gpiocrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOH",
        address: 1073880064,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "gpiohen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "gpiohrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 1073763328,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "i2c1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "i2c1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
    },
    Peripheral {
        name: "I2C2",
        address: 1073764352,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "i2c2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "i2c2rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 1073754112,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdg",
                version: "v1",
                block: "IWDG",
                ir: &iwdg::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1073751040,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "lptim1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "lptim1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "dckcfgr2",
                        field: "lptim1sel",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "OUT",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "OUT",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM1",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 1073770496,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "f4",
                block: "PWR",
                ir: &pwr::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "pwren",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "pwrrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1073887232,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "f410",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PH0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "OSC_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 1074266112,
        registers: Some(
            PeripheralRegisters {
                kind: "rng",
                version: "v1",
                block: "RNG",
                ir: &rng::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "rngen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "rngrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RNG",
            },
        ],
    },
    Peripheral {
        name: "RTC",
        address: 1073752064,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v2f4",
                block: "RTC",
                ir: &rtc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "rtcapben",
                    },
                ),
                reset: None,
                mux: None,
                stop_mode: StopMode::Standby,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "AF1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_ALARM",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "TAMP_STAMP",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "TAMP_STAMP",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "spi1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "spi1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_MCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI1",
            },
        ],
    },
    Peripheral {
        name: "SPI2",
        address: 1073756160,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "spi2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "spi2rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_MCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_MCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI2",
            },
        ],
    },
    Peripheral {
        name: "SPI5",
        address: 1073827840,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "spi5en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "spi5rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_CK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "SCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "I2S_WS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MOSI",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI5",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073821696,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "f4",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "syscfgen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "syscfgrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 1073807360,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_ADV",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "tim1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
        ],
    },
    Peripheral {
        name: "TIM11",
        address: 1073825792,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_GP16",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "tim11en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim11rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
        ],
    },
    Peripheral {
        name: "TIM5",
        address: 1073744896,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_GP32",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "tim5en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "tim5rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 1073745920,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_BASIC",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "tim6en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "tim6rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC",
            },
        ],
    },
    Peripheral {
        name: "TIM9",
        address: 1073823744,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_GP16",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "tim9en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim9rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM9",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 536836624,
        registers: Some(
            PeripheralRegisters {
                kind: "uid",
                version: "v1",
                block: "UID",
                ir: &uid::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 1073811456,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "usart1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "usart1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART1",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 1073759232,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "usart2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "usart2rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART2",
            },
        ],
    },
    Peripheral {
        name: "USART6",
        address: 1073812480,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "usart6en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "usart6rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART6",
            },
        ],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: Some(
            PeripheralRegisters {
                kind: "wwdg",
                version: "v1",
                block: "WWDG",
                ir: &wwdg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr",
                        field: "wwdgen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr",
                        field: "wwdgrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG",
            },
        ],
    },
];
                pub(crate) static INTERRUPTS: &'static [Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt {
        name: "PVD",
        number: 1,
    },
    Interrupt {
        name: "TAMP_STAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP",
        number: 3,
    },
    Interrupt {
        name: "FLASH",
        number: 4,
    },
    Interrupt {
        name: "RCC",
        number: 5,
    },
    Interrupt {
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI1",
        number: 7,
    },
    Interrupt {
        name: "EXTI2",
        number: 8,
    },
    Interrupt {
        name: "EXTI3",
        number: 9,
    },
    Interrupt {
        name: "EXTI4",
        number: 10,
    },
    Interrupt {
        name: "DMA1_STREAM0",
        number: 11,
    },
    Interrupt {
        name: "DMA1_STREAM1",
        number: 12,
    },
    Interrupt {
        name: "DMA1_STREAM2",
        number: 13,
    },
    Interrupt {
        name: "DMA1_STREAM3",
        number: 14,
    },
    Interrupt {
        name: "DMA1_STREAM4",
        number: 15,
    },
    Interrupt {
        name: "DMA1_STREAM5",
        number: 16,
    },
    Interrupt {
        name: "DMA1_STREAM6",
        number: 17,
    },
    Interrupt {
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM9",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM11",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 27,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI1",
        number: 35,
    },
    Interrupt {
        name: "SPI2",
        number: 36,
    },
    Interrupt {
        name: "USART1",
        number: 37,
    },
    Interrupt {
        name: "USART2",
        number: 38,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "DMA1_STREAM7",
        number: 47,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "TIM6_DAC",
        number: 54,
    },
    Interrupt {
        name: "DMA2_STREAM0",
        number: 56,
    },
    Interrupt {
        name: "DMA2_STREAM1",
        number: 57,
    },
    Interrupt {
        name: "DMA2_STREAM2",
        number: 58,
    },
    Interrupt {
        name: "DMA2_STREAM3",
        number: 59,
    },
    Interrupt {
        name: "DMA2_STREAM4",
        number: 60,
    },
    Interrupt {
        name: "DMA2_STREAM5",
        number: 68,
    },
    Interrupt {
        name: "DMA2_STREAM6",
        number: 69,
    },
    Interrupt {
        name: "DMA2_STREAM7",
        number: 70,
    },
    Interrupt {
        name: "USART6",
        number: 71,
    },
    Interrupt {
        name: "RNG",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "SPI5",
        number: 85,
    },
    Interrupt {
        name: "FMPI2C1_EV",
        number: 95,
    },
    Interrupt {
        name: "FMPI2C1_ER",
        number: 96,
    },
    Interrupt {
        name: "LPTIM1",
        number: 97,
    },
];
                pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH0",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
];
            #[path="../registers/adc_v2.rs"] pub mod adc;
#[path="../registers/adccommon_v2.rs"] pub mod adccommon;
#[path="../registers/crc_v1.rs"] pub mod crc;
#[path="../registers/dac_v2.rs"] pub mod dac;
#[path="../registers/dbgmcu_f4.rs"] pub mod dbgmcu;
#[path="../registers/dma_v2.rs"] pub mod dma;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_f4.rs"] pub mod flash;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v1.rs"] pub mod i2c;
#[path="../registers/iwdg_v1.rs"] pub mod iwdg;
#[path="../registers/pwr_f4.rs"] pub mod pwr;
#[path="../registers/rcc_f410.rs"] pub mod rcc;
#[path="../registers/rng_v1.rs"] pub mod rng;
#[path="../registers/rtc_v2f4.rs"] pub mod rtc;
#[path="../registers/spi_v1.rs"] pub mod spi;
#[path="../registers/syscfg_f4.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v2.rs"] pub mod usart;
#[path="../registers/wwdg_v1.rs"] pub mod wwdg;
