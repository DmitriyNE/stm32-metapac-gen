
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "Power control",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "power control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr",
                    description: Some(
                        "power control/status register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "power control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpsdsr",
                    description: Some(
                        "Low-power deepsleep/Sleep/Low-power run",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "pdds",
                    description: Some(
                        "Power down deepsleep",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pdds",
                    ),
                },
                Field {
                    name: "cwuf",
                    description: Some(
                        "Clear wakeup flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csbf",
                    description: Some(
                        "Clear standby flag",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvde",
                    description: Some(
                        "Power voltage detector enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pls",
                    description: Some(
                        "PVD level selection",
                    ),
                    bit_offset: 5,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pls",
                    ),
                },
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable backup domain write protection",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ulp",
                    description: Some(
                        "Ultra-low-power mode",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fwu",
                    description: Some(
                        "Fast wakeup",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vos",
                    description: Some(
                        "Voltage scaling range selection",
                    ),
                    bit_offset: 11,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
                Field {
                    name: "ds_ee_koff",
                    description: Some(
                        "Deep sleep mode with Flash memory kept off",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "DsEeKoff",
                    ),
                },
                Field {
                    name: "lprun",
                    description: Some(
                        "Low power run mode",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "lpds",
                    description: Some(
                        "Regulator in Low-power deepsleep mode",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "power control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf",
                    description: Some(
                        "Wakeup flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbf",
                    description: Some(
                        "Standby flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdo",
                    description: Some(
                        "PVD output",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefintrdyf",
                    description: Some(
                        "Internal voltage reference ready flag",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vosf",
                    description: Some(
                        "Voltage Scaling select flag",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reglpf",
                    description: Some(
                        "Regulator LP flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewup1",
                    description: Some(
                        "Enable WKUP pin 1",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewup2",
                    description: Some(
                        "Enable WKUP pin 2",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewup3",
                    description: Some(
                        "Enable WKUP pin 3",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "DsEeKoff",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NVMWAKEUP",
                    description: Some(
                        "NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NVMSLEEP",
                    description: Some(
                        "NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MAIN_MODE",
                    description: Some(
                        "Voltage regulator in Main mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW_POWER_MODE",
                    description: Some(
                        "Voltage regulator switches to low-power mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pdds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOP_MODE",
                    description: Some(
                        "Enter Stop mode when the CPU enters deepsleep",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STANDBY_MODE",
                    description: Some(
                        "Enter Standby mode when the CPU enters deepsleep",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pls",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "V1_9",
                    description: Some(
                        "1.9 V",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "V2_1",
                    description: Some(
                        "2.1 V",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "V2_3",
                    description: Some(
                        "2.3 V",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "V2_5",
                    description: Some(
                        "2.5 V",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "V2_7",
                    description: Some(
                        "2.7 V",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "V2_9",
                    description: Some(
                        "2.9 V",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "V3_1",
                    description: Some(
                        "3.1 V",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "External input analog voltage (Compare internally to VREFINT)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Vos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "1.8 V (range 1)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "1.5 V (range 2)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE3",
                    description: Some(
                        "1.2 V (range 3)",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                