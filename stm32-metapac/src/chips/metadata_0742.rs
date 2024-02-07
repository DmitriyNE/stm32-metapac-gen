
                pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1342439424,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v3",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "adcen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "IN16",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN14",
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
        address: 1342440192,
        registers: Some(
            PeripheralRegisters {
                kind: "adccommon",
                version: "v3",
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
        name: "AES1",
        address: 1342570496,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "aes1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
                        field: "aes1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    38,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "AES1",
            },
        ],
    },
    Peripheral {
        name: "AES2",
        address: 1476401152,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "aes2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
                        field: "aes2rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    39,
                ),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    40,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "AES2",
            },
        ],
    },
    Peripheral {
        name: "COMP1",
        address: 1073807872,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "COMP",
            },
        ],
    },
    Peripheral {
        name: "COMP2",
        address: 1073807876,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "COMP",
            },
        ],
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
        name: "CRS",
        address: 1073766400,
        registers: Some(
            PeripheralRegisters {
                kind: "crs",
                version: "v1",
                block: "CRS",
                ir: &crs::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "crsen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "crsrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SYNC",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "wb",
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
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073873920,
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
                signal: "CH1",
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CHANNEL7",
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
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "dmamux1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "dmamux1rst",
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
                signal: "OVR",
                interrupt: "DMAMUX1_OVR",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 1476397056,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "w",
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
        address: 1476411392,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "wb",
                block: "FLASH",
                ir: &flash::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "flashen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
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
        address: 1207959552,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "gpioaen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
        address: 1207960576,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "gpioben",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
        address: 1207961600,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "gpiocen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
        address: 1207962624,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "gpioden",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
        name: "GPIOE",
        address: 1207963648,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "gpioeen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
                        field: "gpioerst",
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
        address: 1207966720,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "gpiohen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
        name: "HSEM",
        address: 1476400128,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "hsemen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
                        field: "hsemrst",
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
                interrupt: "HSEM",
            },
        ],
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
                        register: "apb1enr1",
                        field: "i2c1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
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
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
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
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
    },
    Peripheral {
        name: "I2C3",
        address: 1073765376,
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
                        register: "apb1enr1",
                        field: "i2c3en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "i2c3rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "i2c3sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
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
                pin: "PB13",
                signal: "SCL",
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
                pin: "PB2",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDA",
                af: Some(
                    4,
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
                    12,
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
                    13,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
        ],
    },
    Peripheral {
        name: "IPCC",
        address: 1476398080,
        registers: Some(
            PeripheralRegisters {
                kind: "ipcc",
                version: "v1",
                block: "IPCC",
                ir: &ipcc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "ipccen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
                        field: "ipccrst",
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
                signal: "C1_RX",
                interrupt: "IPCC_C1_RX",
            },
            PeripheralInterrupt {
                signal: "C1_TX",
                interrupt: "IPCC_C1_TX",
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
        name: "LCD",
        address: 1073751040,
        registers: Some(
            PeripheralRegisters {
                kind: "lcd",
                version: "v2",
                block: "LCD",
                ir: &lcd::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "lcden",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "lcdrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SEG0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COM2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SEG5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SEG17",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SEG1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SEG2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SEG5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SEG3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SEG4",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "COM0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "COM1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SEG10",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SEG11",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SEG12",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SEG13",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SEG14",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SEG15",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VLCD",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SEG7",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SEG8",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SEG9",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SEG6",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SEG21",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SEG16",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "COM3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SEG18",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SEG19",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "COM4",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SEG28",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SEG40",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "COM5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SEG29",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SEG41",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "COM6",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SEG30",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SEG42",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SEG20",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "VLCD",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SEG22",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SEG23",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SEG24",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SEG25",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SEG26",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SEG27",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SEG30",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SEG31",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SEG32",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SEG33",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "SEG34",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "SEG35",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "COM7",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SEG31",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SEG43",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "SEG39",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "SEG28",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "SEG29",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "SEG36",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "SEG37",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "SEG38",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LCD",
            },
        ],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1073773568,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v1",
                block: "LPTIM",
                ir: &lptim::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "lptim1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "lptim1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "lptim1sel",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "OUT",
                af: Some(
                    1,
                ),
            },
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
                pin: "PC0",
                signal: "IN1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "OUT",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
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
        name: "LPTIM2",
        address: 1073779712,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v1",
                block: "LPTIM",
                ir: &lptim::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr2",
                        field: "lptim2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr2",
                        field: "lptim2rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "lptim2sel",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "OUT",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "OUT",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM2",
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 1073774592,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v4",
                block: "LPUART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr2",
                        field: "lpuart1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr2",
                        field: "lpuart1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "lpuart1sel",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                af: Some(
                    8,
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
                interrupt: "LPUART1",
            },
        ],
    },
    Peripheral {
        name: "PKA",
        address: 1476403200,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "pkaen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
                        field: "pkarst",
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
                interrupt: "PKA",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 1476396032,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "wb55",
                block: "PWR",
                ir: &pwr::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QUADSPI",
        address: 2415919104,
        registers: Some(
            PeripheralRegisters {
                kind: "quadspi",
                version: "v1",
                block: "QUADSPI",
                ir: &quadspi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "quadspien",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
                        field: "quadspirst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "BK1_NCS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CLK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BK1_IO3",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "BK1_IO2",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CLK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "BK1_NCS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "BK1_IO1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "BK1_IO0",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "BK1_NCS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "BK1_IO0",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "BK1_IO1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "BK1_IO2",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "BK1_IO3",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "QUADSPI",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    20,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "QUADSPI",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 1476395008,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "wb",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "MCO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MCO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "LSCO",
                af: Some(
                    6,
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
                pin: "PH3",
                signal: "LSCO",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CRS",
                interrupt: "CRS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
            PeripheralInterrupt {
                signal: "LSECSS",
                interrupt: "TAMP_STAMP_LSECSS",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 1476399104,
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
                clock: "hclk3",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb3enr",
                        field: "rngen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb3rstr",
                        field: "rngrst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "rngsel",
                    },
                ),
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
                version: "v2wb",
                block: "RTC",
                ir: &rtc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
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
                pin: "PA0",
                signal: "TAMP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TAMP3",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMP1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TS",
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
                interrupt: "TAMP_STAMP_LSECSS",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "TAMP_STAMP_LSECSS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SAI1",
        address: 1073828864,
        registers: Some(
            PeripheralRegisters {
                kind: "sai",
                version: "v1",
                block: "SAI",
                ir: &sai::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "sai1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "sai1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "sai1sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "EXTCLK",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FS_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FS_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "EXTCLK",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MCLK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FS_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SCK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "MCLK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    19,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SAI1",
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
                pin: "PA1",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
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
                signal: "NSS",
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
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "NSS",
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
                signal: "MOSI",
                af: Some(
                    5,
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
                    6,
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
                    7,
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
                        register: "apb1enr1",
                        field: "spi2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "spi2rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
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
                signal: "NSS",
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
                signal: "MOSI",
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
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "MOSI",
                af: Some(
                    5,
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
                    8,
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
                    9,
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
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "wb",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
            },
        ),
        rcc: None,
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
                signal: "BKIN2",
                af: Some(
                    12,
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
                    12,
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
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    3,
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
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BKIN",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "BKIN2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
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
                    21,
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
                    22,
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
                    23,
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
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    26,
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
                    27,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM16",
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
                clock: "pclk2_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "tim16en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim16rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "CH1",
                af: Some(
                    14,
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
                    33,
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
                    34,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM16",
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
                clock: "pclk2_tim",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "tim17en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim17rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH1",
                af: Some(
                    14,
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
                    35,
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
                    36,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 1073741824,
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
                        register: "apb1enr1",
                        field: "tim2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "tim2rst",
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
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
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
                    28,
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
                    29,
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
                    30,
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
                    31,
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
                    32,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "TSC",
        address: 1073889280,
        registers: Some(
            PeripheralRegisters {
                kind: "tsc",
                version: "v2",
                block: "TSC",
                ir: &tsc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "tscen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
                        field: "tscrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "G3_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SYNC",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "G1_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "G1_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "G1_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "G1_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "G2_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "G2_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "G2_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "G2_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "G7_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "G3_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "G3_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "G3_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "G4_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "G4_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "G4_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "G4_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "G6_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "G6_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "G6_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "G6_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SYNC",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "G5_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "G5_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "G5_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "G5_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "G7_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "G7_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "G7_IO1",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TSC",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 536835472,
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
                pin: "PA11",
                signal: "NSS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
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
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
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
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    14,
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
                    15,
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
        name: "USB",
        address: 1073768448,
        registers: Some(
            PeripheralRegisters {
                kind: "usb",
                version: "v3",
                block: "USB",
                ir: &usb::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "usben",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "usbrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "NOE",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "NOE",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_HP",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_LP",
            },
        ],
    },
    Peripheral {
        name: "USBRAM",
        address: 1073769472,
        registers: Some(
            PeripheralRegisters {
                kind: "usbram",
                version: "16x2_1024",
                block: "USBRAM",
                ir: &usbram::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "VREFINTCAL",
        address: 536835498,
        registers: Some(
            PeripheralRegisters {
                kind: "vrefintcal",
                version: "v1",
                block: "VREFINTCAL",
                ir: &vrefintcal::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
                        register: "apb1enr1",
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
        name: "PVD_PVM",
        number: 1,
    },
    Interrupt {
        name: "TAMP_STAMP_LSECSS",
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
        name: "DMA1_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 17,
    },
    Interrupt {
        name: "ADC1",
        number: 18,
    },
    Interrupt {
        name: "USB_HP",
        number: 19,
    },
    Interrupt {
        name: "USB_LP",
        number: 20,
    },
    Interrupt {
        name: "C2SEV_PWR_C2H",
        number: 21,
    },
    Interrupt {
        name: "COMP",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM16",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM17",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 27,
    },
    Interrupt {
        name: "TIM2",
        number: 28,
    },
    Interrupt {
        name: "PKA",
        number: 29,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 30,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 31,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 32,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 33,
    },
    Interrupt {
        name: "SPI1",
        number: 34,
    },
    Interrupt {
        name: "SPI2",
        number: 35,
    },
    Interrupt {
        name: "USART1",
        number: 36,
    },
    Interrupt {
        name: "LPUART1",
        number: 37,
    },
    Interrupt {
        name: "SAI1",
        number: 38,
    },
    Interrupt {
        name: "TSC",
        number: 39,
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
        name: "CRS",
        number: 42,
    },
    Interrupt {
        name: "IPCC_C1_RX",
        number: 44,
    },
    Interrupt {
        name: "IPCC_C1_TX",
        number: 45,
    },
    Interrupt {
        name: "HSEM",
        number: 46,
    },
    Interrupt {
        name: "LPTIM1",
        number: 47,
    },
    Interrupt {
        name: "LPTIM2",
        number: 48,
    },
    Interrupt {
        name: "LCD",
        number: 49,
    },
    Interrupt {
        name: "QUADSPI",
        number: 50,
    },
    Interrupt {
        name: "AES1",
        number: 51,
    },
    Interrupt {
        name: "AES2",
        number: 52,
    },
    Interrupt {
        name: "RNG",
        number: 53,
    },
    Interrupt {
        name: "FPU",
        number: 54,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 59,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 60,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 61,
    },
    Interrupt {
        name: "DMAMUX1_OVR",
        number: 62,
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
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            3,
        ),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            4,
        ),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            5,
        ),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            6,
        ),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            7,
        ),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            8,
        ),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            9,
        ),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            10,
        ),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            11,
        ),
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 5,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            12,
        ),
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 6,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            13,
        ),
    },
];
            #[path="../registers/adc_v3.rs"] pub mod adc;
#[path="../registers/adccommon_v3.rs"] pub mod adccommon;
#[path="../registers/bdma_v1.rs"] pub mod bdma;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/crs_v1.rs"] pub mod crs;
#[path="../registers/dbgmcu_wb.rs"] pub mod dbgmcu;
#[path="../registers/dmamux_v1.rs"] pub mod dmamux;
#[path="../registers/exti_w.rs"] pub mod exti;
#[path="../registers/flash_wb.rs"] pub mod flash;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/ipcc_v1.rs"] pub mod ipcc;
#[path="../registers/iwdg_v2.rs"] pub mod iwdg;
#[path="../registers/lcd_v2.rs"] pub mod lcd;
#[path="../registers/lptim_v1.rs"] pub mod lptim;
#[path="../registers/pwr_wb55.rs"] pub mod pwr;
#[path="../registers/quadspi_v1.rs"] pub mod quadspi;
#[path="../registers/rcc_wb.rs"] pub mod rcc;
#[path="../registers/rng_v1.rs"] pub mod rng;
#[path="../registers/rtc_v2wb.rs"] pub mod rtc;
#[path="../registers/sai_v1.rs"] pub mod sai;
#[path="../registers/spi_v2.rs"] pub mod spi;
#[path="../registers/syscfg_wb.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/tsc_v2.rs"] pub mod tsc;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v4.rs"] pub mod usart;
#[path="../registers/usb_v3.rs"] pub mod usb;
#[path="../registers/usbram_16x2_1024.rs"] pub mod usbram;
#[path="../registers/vrefintcal_v1.rs"] pub mod vrefintcal;
#[path="../registers/wwdg_v2.rs"] pub mod wwdg;
