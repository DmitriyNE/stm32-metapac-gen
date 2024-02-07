
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
                pin: "PB0",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
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
                signal: "ADC1",
                channel: Some(
                    "DMA2_CH3",
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
                interrupt: "ADC1_2",
            },
        ],
    },
    Peripheral {
        name: "ADC2",
        address: 1342439680,
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
                pin: "PB0",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
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
                signal: "ADC2",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
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
                interrupt: "ADC1_2",
            },
        ],
    },
    Peripheral {
        name: "ADC3",
        address: 1342439936,
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC3",
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
                signal: "ADC3",
                channel: Some(
                    "DMA2_CH5",
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
                interrupt: "ADC3",
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
        name: "AES",
        address: 1342570496,
        registers: Some(
            PeripheralRegisters {
                kind: "aes",
                version: "v1",
                block: "AES",
                ir: &aes::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "aesen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
                        field: "aesrst",
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
                signal: "OUT",
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
                signal: "OUT",
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
                signal: "IN",
                channel: Some(
                    "DMA2_CH5",
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
                interrupt: "AES",
            },
        ],
    },
    Peripheral {
        name: "CAN1",
        address: 1073767424,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "bxcan",
                block: "CAN",
                ir: &can::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "can1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "can1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN1_TX",
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
                pin: "PB0",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INM",
                af: None,
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
        name: "DAC1",
        address: 1073771520,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v3",
                block: "DAC",
                ir: &dac::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "dac1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "dac1rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
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
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
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
                version: "l4",
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
        name: "DFSDM1",
        address: 1073831936,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr2",
                        field: "dfsdmen",
                    },
                ),
                reset: None,
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "dfsdmsel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "DATIN0",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DATIN7",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CKIN7",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DATIN1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CKIN1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DATIN2",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CKIN2",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CKIN0",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "DATIN5",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CKIN5",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "DATIN6",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CKIN6",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "DATIN4",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CKIN4",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CKOUT",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CKIN3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "DATIN3",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT1",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT2",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT3",
                channel: Some(
                    "DMA1_CH7",
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
                signal: "FLT0",
                interrupt: "DFSDM1_FLT0",
            },
            PeripheralInterrupt {
                signal: "FLT1",
                interrupt: "DFSDM1_FLT1",
            },
            PeripheralInterrupt {
                signal: "FLT2",
                interrupt: "DFSDM1_FLT2",
            },
            PeripheralInterrupt {
                signal: "FLT3",
                interrupt: "DFSDM1_FLT3",
            },
        ],
    },
    Peripheral {
        name: "DMA1",
        address: 1073872896,
        registers: Some(
            PeripheralRegisters {
                kind: "bdma",
                version: "v2",
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
                version: "v2",
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
        name: "EXTI",
        address: 1073808384,
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
        address: 1073881088,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "l4",
                block: "FLASH",
                ir: &flash::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb1enr",
                        field: "flashen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb1rstr",
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
        name: "GPIOF",
        address: 1207964672,
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
                        field: "gpiofen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
        name: "GPIOG",
        address: 1207965696,
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
                        field: "gpiogen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
                        field: "gpiogrst",
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
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
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
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
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
                        field: "i2c2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "i2c2rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "i2c2sel",
                    },
                ),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
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
                    3,
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
                pin: "PB2",
                signal: "SMBA",
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
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
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
                    3,
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
                pin: "PB0",
                signal: "SEG5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SEG6",
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
        registers: None,
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
        registers: None,
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
                version: "v3",
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
                signal: "TX",
                channel: Some(
                    "DMA2_CH6",
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
                interrupt: "LPUART1",
            },
        ],
    },
    Peripheral {
        name: "OPAMP1",
        address: 1073772544,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "opampen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "opamprst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VOUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP2",
        address: 1073772560,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VOUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWR",
        address: 1073770496,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "l4",
                block: "PWR",
                ir: &pwr::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "pwren",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
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
        name: "QUADSPI",
        address: 2684358656,
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
                pin: "PB0",
                signal: "BK1_IO1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "BK1_IO0",
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
                signal: "NCS",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "QUADSPI",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "QUADSPI",
                channel: Some(
                    "DMA2_CH7",
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
                interrupt: "QUADSPI",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 1073876992,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "l4",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
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
        address: 1342572544,
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
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "rngen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
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
                version: "v2l4",
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
                signal: "OUT_ALARM",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT_CALIB",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT_ALARM",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT_CALIB",
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
        name: "SAI1",
        address: 1073828864,
        registers: Some(
            PeripheralRegisters {
                kind: "sai",
                version: "v2",
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
                pin: "PA4",
                signal: "FS_B",
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
                signal: "MCLK_A",
                af: Some(
                    13,
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
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some(
                    "DMA2_CH7",
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
                signal: "GLOBAL",
                interrupt: "SAI1",
            },
        ],
    },
    Peripheral {
        name: "SAI2",
        address: 1073829888,
        registers: Some(
            PeripheralRegisters {
                kind: "sai",
                version: "v2",
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
                        field: "sai2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "sai2rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "sai2sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "EXTCLK",
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
                pin: "PC10",
                signal: "SCK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MCLK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "MCLK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "EXTCLK",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
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
                signal: "B",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some(
                    "DMA2_CH4",
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
                signal: "GLOBAL",
                interrupt: "SAI2",
            },
        ],
    },
    Peripheral {
        name: "SDMMC1",
        address: 1073817600,
        registers: Some(
            PeripheralRegisters {
                kind: "sdmmc",
                version: "v1",
                block: "SDMMC",
                ir: &sdmmc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb2enr",
                        field: "sdmmcen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "sdmmcrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SDMMC1",
                channel: Some(
                    "DMA2_CH4",
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
                    "DMA2_CH4",
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
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "SDMMC1",
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
                    "DMA2_CH5",
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
                interrupt: "SDMMC1",
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
                channel: Some(
                    "DMA1_CH2",
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
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH3",
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
                    "DMA2_CH4",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
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
                    "DMA1_CH5",
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
                signal: "GLOBAL",
                interrupt: "SPI2",
            },
        ],
    },
    Peripheral {
        name: "SPI3",
        address: 1073757184,
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
                        field: "spi3en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "spi3rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC12",
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
                    "DMA2_CH1",
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
                    3,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI3",
            },
        ],
    },
    Peripheral {
        name: "SWPMI1",
        address: 1073776640,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr2",
                        field: "swpmi1en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr2",
                        field: "swpmi1rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "swpmi1sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "IO",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RX",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SUSPEND",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SWPMI1",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "l4",
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
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2_COMP1",
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
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN_COMP2",
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
                pin: "PB12",
                signal: "BKIN_COMP2",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
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
                signal: "CH2",
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
                signal: "CH4",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
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
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
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
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM15",
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
        name: "TIM15",
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
                        field: "tim15en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim15rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(
                    14,
                ),
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
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
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
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM15",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
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
                signal: "UP",
                channel: Some(
                    "DMA1_CH6",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH7",
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
                signal: "CH3",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
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
                signal: "CH2",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
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
                        register: "apb1enr1",
                        field: "tim3en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "tim3rst",
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
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
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
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH6",
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
        name: "TIM4",
        address: 1073743872,
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
                        register: "apb1enr1",
                        field: "tim4en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "tim4rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
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
                signal: "CH3",
                channel: Some(
                    "DMA1_CH5",
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
                signal: "BRK",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
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
                        register: "apb1enr1",
                        field: "tim5en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH4",
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
                signal: "TRIG",
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
                signal: "COM",
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
                signal: "CH3",
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
                signal: "UP",
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
                signal: "CH2",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH5",
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
                        register: "apb1enr1",
                        field: "tim6en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
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
                    "DMA1_CH3",
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
                    "DMA2_CH4",
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
        name: "TIM7",
        address: 1073746944,
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
                        register: "apb1enr1",
                        field: "tim7en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "tim7rst",
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
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
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
                    3,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
        ],
    },
    Peripheral {
        name: "TIM8",
        address: 1073820672,
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
                        field: "tim8en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb2rstr",
                        field: "tim8rst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN_COMP2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN2_COMP2",
                af: Some(
                    12,
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
                pin: "PB7",
                signal: "BKIN_COMP1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BKIN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BKIN2_COMP1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
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
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH7",
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
                interrupt: "TIM8_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP",
            },
        ],
    },
    Peripheral {
        name: "TSC",
        address: 1073889280,
        registers: Some(
            PeripheralRegisters {
                kind: "tsc",
                version: "v3",
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
                pin: "PD2",
                signal: "SYNC",
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
        name: "UART4",
        address: 1073761280,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "uart4en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "uart4rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "uart4sel",
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
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
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
                signal: "RX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART4",
            },
        ],
    },
    Peripheral {
        name: "UART5",
        address: 1073762304,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "uart5en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "uart5rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "uart5sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH1",
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
                    2,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART5",
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
                version: "v3",
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
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
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
                    "DMA1_CH5",
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
                    "DMA2_CH6",
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
                    "DMA2_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
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
                version: "v3",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "usart2en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "usart2rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "usart2sel",
                    },
                ),
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
                signal: "DE",
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
                    "DMA1_CH6",
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
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
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
        name: "USART3",
        address: 1073760256,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "pclk1",
                enable: Some(
                    PeripheralRccRegister {
                        register: "apb1enr1",
                        field: "usart3en",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "apb1rstr1",
                        field: "usart3rst",
                    },
                ),
                mux: Some(
                    PeripheralRccRegister {
                        register: "ccipr",
                        field: "usart3sel",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
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
                    2,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART3",
            },
        ],
    },
    Peripheral {
        name: "USB_OTG_FS",
        address: 1342177280,
        registers: Some(
            PeripheralRegisters {
                kind: "otg",
                version: "v1",
                block: "OTG",
                ir: &otg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                clock: "hclk2",
                enable: Some(
                    PeripheralRccRegister {
                        register: "ahb2enr",
                        field: "usb_otg_fsen",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "ahb2rstr",
                        field: "usb_otg_fsrst",
                    },
                ),
                mux: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "ID",
                af: Some(
                    10,
                ),
            },
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
                pin: "PA8",
                signal: "SOF",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "VBUS",
                af: None,
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
                signal: "EP1_IN",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_FS",
            },
        ],
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
        name: "ADC1_2",
        number: 18,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM15",
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
        name: "TIM3",
        number: 29,
    },
    Interrupt {
        name: "TIM4",
        number: 30,
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
        name: "USART3",
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
        name: "DFSDM1_FLT3",
        number: 42,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "ADC3",
        number: 47,
    },
    Interrupt {
        name: "FMC",
        number: 48,
    },
    Interrupt {
        name: "SDMMC1",
        number: 49,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "UART4",
        number: 52,
    },
    Interrupt {
        name: "UART5",
        number: 53,
    },
    Interrupt {
        name: "TIM6_DAC",
        number: 54,
    },
    Interrupt {
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 59,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 60,
    },
    Interrupt {
        name: "DFSDM1_FLT0",
        number: 61,
    },
    Interrupt {
        name: "DFSDM1_FLT1",
        number: 62,
    },
    Interrupt {
        name: "DFSDM1_FLT2",
        number: 63,
    },
    Interrupt {
        name: "COMP",
        number: 64,
    },
    Interrupt {
        name: "LPTIM1",
        number: 65,
    },
    Interrupt {
        name: "LPTIM2",
        number: 66,
    },
    Interrupt {
        name: "OTG_FS",
        number: 67,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 68,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 69,
    },
    Interrupt {
        name: "LPUART1",
        number: 70,
    },
    Interrupt {
        name: "QUADSPI",
        number: 71,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 72,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 73,
    },
    Interrupt {
        name: "SAI1",
        number: 74,
    },
    Interrupt {
        name: "SAI2",
        number: 75,
    },
    Interrupt {
        name: "SWPMI1",
        number: 76,
    },
    Interrupt {
        name: "TSC",
        number: 77,
    },
    Interrupt {
        name: "LCD",
        number: 78,
    },
    Interrupt {
        name: "AES",
        number: 79,
    },
    Interrupt {
        name: "RNG",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
];
                pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
];
            #[path="../registers/adc_v3.rs"] pub mod adc;
#[path="../registers/adccommon_v3.rs"] pub mod adccommon;
#[path="../registers/aes_v1.rs"] pub mod aes;
#[path="../registers/bdma_v2.rs"] pub mod bdma;
#[path="../registers/can_bxcan.rs"] pub mod can;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/dac_v3.rs"] pub mod dac;
#[path="../registers/dbgmcu_l4.rs"] pub mod dbgmcu;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_l4.rs"] pub mod flash;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/iwdg_v2.rs"] pub mod iwdg;
#[path="../registers/lcd_v2.rs"] pub mod lcd;
#[path="../registers/otg_v1.rs"] pub mod otg;
#[path="../registers/pwr_l4.rs"] pub mod pwr;
#[path="../registers/quadspi_v1.rs"] pub mod quadspi;
#[path="../registers/rcc_l4.rs"] pub mod rcc;
#[path="../registers/rng_v1.rs"] pub mod rng;
#[path="../registers/rtc_v2l4.rs"] pub mod rtc;
#[path="../registers/sai_v2.rs"] pub mod sai;
#[path="../registers/sdmmc_v1.rs"] pub mod sdmmc;
#[path="../registers/spi_v2.rs"] pub mod spi;
#[path="../registers/syscfg_l4.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/tsc_v3.rs"] pub mod tsc;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v3.rs"] pub mod usart;
#[path="../registers/vrefintcal_v1.rs"] pub mod vrefintcal;
#[path="../registers/wwdg_v1.rs"] pub mod wwdg;
