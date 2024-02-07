
                pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073816576,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "adcen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "adcrst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "adcsel",
                    },
                ),
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
                pin: "PA11",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "IN14",
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
                pin: "PA8",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN18",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN19",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    5,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC1",
            },
        ],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073817352,
        registers: None,
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
                version: "v3",
                block: "CRC",
                ir: &crc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahbenr",
                        field: "crcen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahbrstr",
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
        name: "DBGMCU",
        address: 1073829888,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "c0",
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
        address: 1073872896,
        registers: Some(
            PeripheralRegisters {
                kind: "bdma",
                version: "v1",
                block: "DMA",
                ir: &bdma::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahbenr",
                        field: "dma1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahbrstr",
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
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2_3",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL2_3",
            },
        ],
    },
    Peripheral {
        name: "DMAMUX1",
        address: 1073874944,
        registers: Some(
            PeripheralRegisters {
                kind: "dmamux",
                version: "v1",
                block: "DMAMUX",
                ir: &dmamux::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 1073879040,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "c0",
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
                interrupt: "EXTI0_1",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI0_1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2_3",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI2_3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI4_15",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "c0",
                block: "FLASH",
                ir: &flash::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahbenr",
                        field: "flashen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahbrstr",
                        field: "flashrst",
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
                interrupt: "FLASH",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 1342177280,
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
                clock: "gpio",
                enable: Some(
                    PeripheralRccRegister {
                        register: "gpioenr",
                        field: "gpioaen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "gpiorstr",
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
        address: 1342178304,
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
                clock: "gpio",
                enable: Some(
                    PeripheralRccRegister {
                        register: "gpioenr",
                        field: "gpioben",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "gpiorstr",
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
        address: 1342179328,
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
                clock: "gpio",
                enable: Some(
                    PeripheralRccRegister {
                        register: "gpioenr",
                        field: "gpiocen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "gpiorstr",
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
        name: "GPIOD",
        address: 1342180352,
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
                clock: "gpio",
                enable: Some(
                    PeripheralRccRegister {
                        register: "gpioenr",
                        field: "gpioden",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "gpiorstr",
                        field: "gpiodrst",
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
        name: "GPIOF",
        address: 1342182400,
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
                clock: "gpio",
                enable: Some(
                    PeripheralRccRegister {
                        register: "gpioenr",
                        field: "gpiofen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "gpiorstr",
                        field: "gpiofrst",
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
                version: "v2",
                block: "I2C",
                ir: &i2c::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr1",
                        field: "i2c1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr1",
                        field: "i2c1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "i2c1sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SMBA",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SCL",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "SDA",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    11,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 1073754112,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdg",
                version: "v2",
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
        name: "PWR",
        address: 1073770496,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "c0",
                block: "PWR",
                ir: &pwr::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr1",
                        field: "pwren",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr1",
                        field: "pwrrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "WKUP3",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1073876992,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "c0",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MCO_2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "MCO_2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MCO_2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_2",
                af: Some(
                    15,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MCO_1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MCO_2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSCX_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_EN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSCX_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC_EN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MCO_1",
                af: Some(
                    0,
                ),
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
        name: "RTC",
        address: 1073752064,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr1",
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
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TS",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "REFIN",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "SSRU",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "spi1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "spi1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "I2S_CK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_MCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_SD",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "I2S_WS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "NSS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "I2S_SD",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_MCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SD",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_WS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "NSS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_WS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_MCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "I2S_CK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "I2S_MCK",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "I2S_SD",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MISO",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCK",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    17,
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
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "c0",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "syscfgen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
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
        address: 1073818624,
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
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "tim1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "tim1rst",
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
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH3N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2N",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH3N",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BKIN2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    21,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    23,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    24,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    25,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
        ],
    },
    Peripheral {
        name: "TIM14",
        address: 1073750016,
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
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "tim14en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "tim14rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM14",
            },
        ],
    },
    Peripheral {
        name: "TIM16",
        address: 1073824768,
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
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "tim16en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "tim16rst",
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
                pin: "PA2",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    46,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM16",
            },
        ],
    },
    Peripheral {
        name: "TIM17",
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
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "tim17en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "tim17rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN",
                af: Some(
                    15,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CH1",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    47,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    48,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    49,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM17",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 1073742848,
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
                clock: "pclk1_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr1",
                        field: "tim3en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr1",
                        field: "tim3rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "ETR",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "ETR",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH4",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CH2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    37,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 536835408,
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
        address: 1073821696,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v4",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr2",
                        field: "usart1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr2",
                        field: "usart1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "usart1sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CK",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "DE",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "DE",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "TX",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    50,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    51,
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
                version: "v4",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr1",
                        field: "usart2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apbrstr1",
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
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NSS",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "NSS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "DE",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CK",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "DE",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RTS",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    52,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    53,
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
        name: "WWDG",
        address: 1073753088,
        registers: Some(
            PeripheralRegisters {
                kind: "wwdg",
                version: "v2",
                block: "WWDG",
                ir: &wwdg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apbenr1",
                        field: "wwdgen",
                    },
                ),
                reset: None,
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
        name: "RTC",
        number: 2,
    },
    Interrupt {
        name: "FLASH",
        number: 3,
    },
    Interrupt {
        name: "RCC",
        number: 4,
    },
    Interrupt {
        name: "EXTI0_1",
        number: 5,
    },
    Interrupt {
        name: "EXTI2_3",
        number: 6,
    },
    Interrupt {
        name: "EXTI4_15",
        number: 7,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 9,
    },
    Interrupt {
        name: "DMA1_CHANNEL2_3",
        number: 10,
    },
    Interrupt {
        name: "DMAMUX1",
        number: 11,
    },
    Interrupt {
        name: "ADC1",
        number: 12,
    },
    Interrupt {
        name: "TIM1_BRK_UP_TRG_COM",
        number: 13,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 14,
    },
    Interrupt {
        name: "TIM3",
        number: 16,
    },
    Interrupt {
        name: "TIM14",
        number: 19,
    },
    Interrupt {
        name: "TIM16",
        number: 21,
    },
    Interrupt {
        name: "TIM17",
        number: 22,
    },
    Interrupt {
        name: "I2C1",
        number: 23,
    },
    Interrupt {
        name: "SPI1",
        number: 25,
    },
    Interrupt {
        name: "USART1",
        number: 27,
    },
    Interrupt {
        name: "USART2",
        number: 28,
    },
];
                pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            0,
        ),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            1,
        ),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            2,
        ),
    },
];
            #[path="../registers/bdma_v1.rs"] pub mod bdma;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/dbgmcu_c0.rs"] pub mod dbgmcu;
#[path="../registers/dmamux_v1.rs"] pub mod dmamux;
#[path="../registers/exti_c0.rs"] pub mod exti;
#[path="../registers/flash_c0.rs"] pub mod flash;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/iwdg_v2.rs"] pub mod iwdg;
#[path="../registers/pwr_c0.rs"] pub mod pwr;
#[path="../registers/rcc_c0.rs"] pub mod rcc;
#[path="../registers/spi_v2.rs"] pub mod spi;
#[path="../registers/syscfg_c0.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v4.rs"] pub mod usart;
#[path="../registers/wwdg_v2.rs"] pub mod wwdg;
