
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Afio",
            extends: None,
            description: Some(
                "Alternate function I/O",
            ),
            items: &[
                BlockItem {
                    name: "evcr",
                    description: Some(
                        "Event Control Register (AFIO_EVCR)",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Evcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mapr",
                    description: Some(
                        "AF remap and debug I/O configuration register (AFIO_MAPR)",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mapr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "External interrupt configuration register 1 (AFIO_EXTICR1)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exticr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mapr2",
                    description: Some(
                        "AF remap and debug I/O configuration register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mapr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Evcr",
            extends: None,
            description: Some(
                "Event Control Register (AFIO_EVCR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin",
                    description: Some(
                        "Pin selection",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "port",
                    description: Some(
                        "Port selection",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "evoe",
                    description: Some(
                        "Event Output Enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some(
                "External interrupt configuration register 3 (AFIO_EXTICR3)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI12 configuration",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mapr",
            extends: None,
            description: Some(
                "AF remap and debug I/O configuration register (AFIO_MAPR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1_remap",
                    description: Some(
                        "SPI1 remapping",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_remap",
                    description: Some(
                        "I2C1 remapping",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1_remap",
                    description: Some(
                        "USART1 remapping",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2_remap",
                    description: Some(
                        "USART2 remapping",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3_remap",
                    description: Some(
                        "USART3 remapping",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_remap",
                    description: Some(
                        "TIM1 remapping",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim2_remap",
                    description: Some(
                        "TIM2 remapping",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3_remap",
                    description: Some(
                        "TIM3 remapping",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4_remap",
                    description: Some(
                        "TIM4 remapping",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can1_remap",
                    description: Some(
                        "CAN1 remapping",
                    ),
                    bit_offset: 13,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd01_remap",
                    description: Some(
                        "Port D0/Port D1 mapping on OSCIN/OSCOUT",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5ch4_iremap",
                    description: Some(
                        "Set and cleared by software",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1_etrginj_remap",
                    description: Some(
                        "ADC 1 External trigger injected conversion remapping",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1_etrgreg_remap",
                    description: Some(
                        "ADC 1 external trigger regular conversion remapping",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2_etrginj_remap",
                    description: Some(
                        "ADC 2 external trigger injected conversion remapping",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2_etrgreg_remap",
                    description: Some(
                        "ADC 2 external trigger regular conversion remapping",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eth_remap",
                    description: Some(
                        "Ethernet MAC I/O remapping",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2_remap",
                    description: Some(
                        "CAN2 I/O remapping",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mii_rmii_sel",
                    description: Some(
                        "MII or RMII selection",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swj_cfg",
                    description: Some(
                        "Serial wire JTAG configuration",
                    ),
                    bit_offset: 24,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3_remap",
                    description: Some(
                        "SPI3/I2S3 remapping",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim2itr1_iremap",
                    description: Some(
                        "TIM2 internal trigger 1 remapping",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ptp_pps_remap",
                    description: Some(
                        "Ethernet PTP PPS remapping",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mapr2",
            extends: None,
            description: Some(
                "AF remap and debug I/O configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim15_remap",
                    description: Some(
                        "TIM15 remapping",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16_remap",
                    description: Some(
                        "TIM16 remapping",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17_remap",
                    description: Some(
                        "TIM17 remapping",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cec_remap",
                    description: Some(
                        "CEC remapping",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_dma_remap",
                    description: Some(
                        "TIM1 DMA remapping",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9_remap",
                    description: Some(
                        "TIM9 remapping",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10_remap",
                    description: Some(
                        "TIM10 remapping",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim11_remap",
                    description: Some(
                        "TIM11 remapping",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13_remap",
                    description: Some(
                        "TIM13 remapping",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14_remap",
                    description: Some(
                        "TIM14 remapping",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fsmc_nadv",
                    description: Some(
                        "NADV connect/disconnect",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim67_dac_dma_remap",
                    description: Some(
                        "TIM67_DAC DMA remapping",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12_remap",
                    description: Some(
                        "TIM12 remapping",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "misc_remap",
                    description: Some(
                        "Miscellaneous features remapping",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                