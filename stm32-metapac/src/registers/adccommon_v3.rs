
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "AdcCommon",
            extends: None,
            description: Some(
                "Analog-to-Digital Converter",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "ADC Common status register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "ADC common control register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdr",
                    description: Some(
                        "ADC common regular data register for dual and triple modes",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "ADC common control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mult",
                    description: Some(
                        "Multi ADC mode selection",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "delay",
                    description: Some(
                        "Delay between 2 sampling phases",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmacfg",
                    description: Some(
                        "DMA configuration (for multi-ADC mode)",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdma",
                    description: Some(
                        "Direct memory access mode for multi ADC mode",
                    ),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckmode",
                    description: Some(
                        "ADC clock mode",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREFINT enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch18sel",
                    description: Some(
                        "CH18 selection (Vbat)",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch17sel",
                    description: Some(
                        "CH17 selection (temperature)",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cdr",
            extends: None,
            description: Some(
                "ADC common regular data register for dual and triple modes",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata_mst",
                    description: Some(
                        "Regular data of the master ADC",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdata_slv",
                    description: Some(
                        "Regular data of the slave ADC",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "ADC Common status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrdy_mst",
                    description: Some(
                        "ADDRDY_MST",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmp_mst",
                    description: Some(
                        "EOSMP_MST",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc_mst",
                    description: Some(
                        "EOC_MST",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eos_mst",
                    description: Some(
                        "EOS_MST",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr_mst",
                    description: Some(
                        "OVR_MST",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc_mst",
                    description: Some(
                        "JEOC_MST",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeos_mst",
                    description: Some(
                        "JEOS_MST",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd_mst",
                    description: Some(
                        "Analog watchdog flag of the master ADC",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jqovf_mst",
                    description: Some(
                        "JQOVF_MST",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adrdy_slv",
                    description: Some(
                        "ADRDY_SLV",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmp_slv",
                    description: Some(
                        "EOSMP_SLV",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc_slv",
                    description: Some(
                        "End of regular conversion of the slave ADC",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eos_slv",
                    description: Some(
                        "End of regular sequence flag of the slave ADC",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr_slv",
                    description: Some(
                        "Overrun flag of the slave ADC",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc_slv",
                    description: Some(
                        "End of injected conversion flag of the slave ADC",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeos_slv",
                    description: Some(
                        "End of injected sequence flag of the slave ADC",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd_slv",
                    description: Some(
                        "Analog watchdog 1 flag of the slave ADC",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jqovf_slv",
                    description: Some(
                        "Injected Context Queue Overflow flag of the slave ADC",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                