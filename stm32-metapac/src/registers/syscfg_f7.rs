
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller",
            ),
            items: &[
                BlockItem {
                    name: "memrmp",
                    description: Some(
                        "memory remap register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Memrmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pmc",
                    description: Some(
                        "peripheral mode configuration register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "external interrupt configuration register 1",
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
                    name: "cmpcr",
                    description: Some(
                        "Compensation cell control register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmpcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cmpcr",
            extends: None,
            description: Some(
                "Compensation cell control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp_pd",
                    description: Some(
                        "Compensation cell power-down",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ready",
                    description: Some(
                        "READY",
                    ),
                    bit_offset: 8,
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
                "external interrupt configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI x configuration (x = 0 to 3)",
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
            name: "Memrmp",
            extends: None,
            description: Some(
                "memory remap register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_boot",
                    description: Some(
                        "Memory boot mapping",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fb_mode",
                    description: Some(
                        "Flash bank mode selection",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swp_fmc",
                    description: Some(
                        "FMC memory mapping swap",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pmc",
            extends: None,
            description: Some(
                "peripheral mode configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c1_fmp",
                    description: Some(
                        "I2C1_FMP I2C1 Fast Mode + Enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2_fmp",
                    description: Some(
                        "I2C2_FMP I2C2 Fast Mode + Enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3_fmp",
                    description: Some(
                        "I2C3_FMP I2C3 Fast Mode + Enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c4_fmp",
                    description: Some(
                        "I2C4 Fast Mode + Enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb6_fmp",
                    description: Some(
                        "PB6_FMP Fast Mode",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb7_fmp",
                    description: Some(
                        "PB7_FMP Fast Mode + Enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb8_fmp",
                    description: Some(
                        "PB8_FMP Fast Mode + Enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb9_fmp",
                    description: Some(
                        "Fast Mode + Enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1dc2",
                    description: Some(
                        "ADC3DC2",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2dc2",
                    description: Some(
                        "ADC2DC2",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc3dc2",
                    description: Some(
                        "ADC3DC2",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mii_rmii_sel",
                    description: Some(
                        "Ethernet PHY interface selection",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                