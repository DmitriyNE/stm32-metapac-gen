
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "TimAdv",
            extends: Some(
                "TIM_GP16",
            ),
            description: Some(
                "Advanced-timers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1Gp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Adv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcr",
                    description: Some(
                        "slave mode control register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierAdv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrAdv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "EgrAdv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrInput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrOutput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcerAdv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Arr16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcr",
                    description: Some(
                        "repetition counter register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdtr",
                    description: Some(
                        "break and dead-time register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmar",
                    description: Some(
                        "DMA address for full transfer",
                    ),
                    array: None,
                    byte_offset: 76,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmar",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimBasic",
            extends: None,
            description: Some(
                "Basic timer",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1Basic",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Basic",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierBasic",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrBasic",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "EgrBasic",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Arr16",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimGp16",
            extends: Some(
                "TIM_BASIC",
            ),
            description: Some(
                "General purpose 16-bit timer",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1Gp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Gp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcr",
                    description: Some(
                        "slave mode control register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "EgrGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrInput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrOutput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcerGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Arr16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmar",
                    description: Some(
                        "DMA address for full transfer",
                    ),
                    array: None,
                    byte_offset: 76,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmar",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TimGp32",
            extends: Some(
                "TIM_GP16",
            ),
            description: Some(
                "General purpose 32-bit timer",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1Gp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Gp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcr",
                    description: Some(
                        "slave mode control register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "DMA/Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DierGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egr",
                    description: Some(
                        "event generation register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "EgrGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_input",
                    description: Some(
                        "capture/compare mode register 1 (input mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrInput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccmr_output",
                    description: Some(
                        "capture/compare mode register 1 (output mode)",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcmrOutput",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccer",
                    description: Some(
                        "capture/compare enable register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CcerGp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "counter",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psc",
                    description: Some(
                        "prescaler",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "auto-reload register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Arr32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "capture/compare register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmar",
                    description: Some(
                        "DMA address for full transfer",
                    ),
                    array: None,
                    byte_offset: 76,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmar",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Arr16",
            extends: None,
            description: Some(
                "auto-reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arr",
                    description: Some(
                        "Auto-reload value",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Arr32",
            extends: None,
            description: Some(
                "auto-reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arr",
                    description: Some(
                        "Auto-reload value",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdtr",
            extends: None,
            description: Some(
                "break and dead-time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtg",
                    description: Some(
                        "Dead-time generator setup",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "Lock configuration",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ossi",
                    description: Some(
                        "Off-state selection for Idle mode",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ossi",
                    ),
                },
                Field {
                    name: "ossr",
                    description: Some(
                        "Off-state selection for Run mode",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ossr",
                    ),
                },
                Field {
                    name: "bke",
                    description: Some(
                        "Break enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkp",
                    description: Some(
                        "Break polarity",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoe",
                    description: Some(
                        "Automatic output enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "moe",
                    description: Some(
                        "Main output enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CcerAdv",
            extends: Some(
                "CCER_GP",
            ),
            description: Some(
                "capture/compare enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare 1 output enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
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
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare 1 output Polarity",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
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
                Field {
                    name: "ccne",
                    description: Some(
                        "Capture/Compare 1 complementary output enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
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
                Field {
                    name: "ccnp",
                    description: Some(
                        "Capture/Compare 1 output Polarity",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
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
            name: "CcerGp",
            extends: None,
            description: Some(
                "capture/compare enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cce",
                    description: Some(
                        "Capture/Compare 1 output enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
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
                Field {
                    name: "ccp",
                    description: Some(
                        "Capture/Compare 1 output Polarity",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
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
                Field {
                    name: "ccnp",
                    description: Some(
                        "Capture/Compare 1 output Polarity",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
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
            name: "CcmrInput",
            extends: None,
            description: Some(
                "capture/compare mode register 1 (input mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare 1 selection",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrInputCcs",
                    ),
                },
                Field {
                    name: "icpsc",
                    description: Some(
                        "Input capture 1 prescaler",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "icf",
                    description: Some(
                        "Input capture 1 filter",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Icf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "CcmrOutput",
            extends: None,
            description: Some(
                "capture/compare mode register 2 (output mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccs",
                    description: Some(
                        "Capture/Compare 3 selection",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "CcmrOutputCcs",
                    ),
                },
                Field {
                    name: "ocfe",
                    description: Some(
                        "Output compare 3 fast enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocpe",
                    description: Some(
                        "Output compare 3 preload enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocm",
                    description: Some(
                        "Output compare 3 mode",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ocm",
                    ),
                },
                Field {
                    name: "occe",
                    description: Some(
                        "Output compare 3 clear enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccr16",
            extends: None,
            description: Some(
                "capture/compare register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccr",
                    description: Some(
                        "Capture/Compare 1 value",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccr32",
            extends: None,
            description: Some(
                "capture/compare register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccr",
                    description: Some(
                        "Capture/Compare 1 value",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cnt16",
            extends: None,
            description: Some(
                "counter",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "counter value",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cnt32",
            extends: None,
            description: Some(
                "counter",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "counter value",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1Basic",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udis",
                    description: Some(
                        "Update disable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "urs",
                    description: Some(
                        "Update request source",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Urs",
                    ),
                },
                Field {
                    name: "opm",
                    description: Some(
                        "One-pulse mode enbaled",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arpe",
                    description: Some(
                        "Auto-reload preload enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1Gp",
            extends: Some(
                "CR1_BASIC",
            ),
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udis",
                    description: Some(
                        "Update disable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "urs",
                    description: Some(
                        "Update request source",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Urs",
                    ),
                },
                Field {
                    name: "opm",
                    description: Some(
                        "One-pulse mode enbaled",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir",
                    description: Some(
                        "Direction",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "cms",
                    description: Some(
                        "Center-aligned mode selection",
                    ),
                    bit_offset: 5,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cms",
                    ),
                },
                Field {
                    name: "arpe",
                    description: Some(
                        "Auto-reload preload enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckd",
                    description: Some(
                        "Clock division",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2Adv",
            extends: Some(
                "CR2_GP",
            ),
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccpc",
                    description: Some(
                        "Capture/compare preloaded control",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccus",
                    description: Some(
                        "Capture/compare control update selection",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccds",
                    description: Some(
                        "Capture/compare DMA selection",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccds",
                    ),
                },
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
                Field {
                    name: "ti1s",
                    description: Some(
                        "TI1 selection",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tis",
                    ),
                },
                Field {
                    name: "ois",
                    description: Some(
                        "Output Idle state 1",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ois1n",
                    description: Some(
                        "Output Idle state 1",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ois2n",
                    description: Some(
                        "Output Idle state 2",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ois3n",
                    description: Some(
                        "Output Idle state 3",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2Basic",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2Gp",
            extends: Some(
                "CR2_BASIC",
            ),
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccds",
                    description: Some(
                        "Capture/compare DMA selection",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ccds",
                    ),
                },
                Field {
                    name: "mms",
                    description: Some(
                        "Master mode selection",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mms",
                    ),
                },
                Field {
                    name: "ti1s",
                    description: Some(
                        "TI1 selection",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tis",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dcr",
            extends: None,
            description: Some(
                "DMA control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dba",
                    description: Some(
                        "DMA base address",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbl",
                    description: Some(
                        "DMA burst length",
                    ),
                    bit_offset: 8,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DierAdv",
            extends: Some(
                "DIER_GP",
            ),
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccie",
                    description: Some(
                        "Capture/Compare 1 interrupt enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comie",
                    description: Some(
                        "COM interrupt enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tie",
                    description: Some(
                        "Trigger interrupt enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bie",
                    description: Some(
                        "Break interrupt enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ude",
                    description: Some(
                        "Update DMA request enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccde",
                    description: Some(
                        "Capture/Compare 1 DMA request enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comde",
                    description: Some(
                        "COM DMA request enable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tde",
                    description: Some(
                        "Trigger DMA request enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DierBasic",
            extends: None,
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ude",
                    description: Some(
                        "Update DMA request enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DierGp",
            extends: Some(
                "DIER_BASIC",
            ),
            description: Some(
                "DMA/Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uie",
                    description: Some(
                        "Update interrupt enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccie",
                    description: Some(
                        "Capture/Compare 1 interrupt enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tie",
                    description: Some(
                        "Trigger interrupt enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ude",
                    description: Some(
                        "Update DMA request enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccde",
                    description: Some(
                        "Capture/Compare 1 DMA request enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tde",
                    description: Some(
                        "Trigger DMA request enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmar",
            extends: None,
            description: Some(
                "DMA address for full transfer",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmab",
                    description: Some(
                        "DMA register for burst accesses",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EgrAdv",
            extends: Some(
                "EGR_GP",
            ),
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccg",
                    description: Some(
                        "Capture/compare 1 generation",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comg",
                    description: Some(
                        "Capture/Compare control update generation",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tg",
                    description: Some(
                        "Trigger generation",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bg",
                    description: Some(
                        "Break generation",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EgrBasic",
            extends: None,
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EgrGp",
            extends: Some(
                "EGR_BASIC",
            ),
            description: Some(
                "event generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ug",
                    description: Some(
                        "Update generation",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccg",
                    description: Some(
                        "Capture/compare 1 generation",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comg",
                    description: Some(
                        "Capture/Compare control update generation",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tg",
                    description: Some(
                        "Trigger generation",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bg",
                    description: Some(
                        "Break generation",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Psc",
            extends: None,
            description: Some(
                "prescaler",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "psc",
                    description: Some(
                        "Prescaler value",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rcr",
            extends: None,
            description: Some(
                "repetition counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rep",
                    description: Some(
                        "Repetition counter value",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Smcr",
            extends: None,
            description: Some(
                "slave mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sms",
                    description: Some(
                        "Slave mode selection",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sms",
                    ),
                },
                Field {
                    name: "ts",
                    description: Some(
                        "Trigger selection",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ts",
                    ),
                },
                Field {
                    name: "msm",
                    description: Some(
                        "Master/Slave mode",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msm",
                    ),
                },
                Field {
                    name: "etf",
                    description: Some(
                        "External trigger filter",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Etf",
                    ),
                },
                Field {
                    name: "etps",
                    description: Some(
                        "External trigger prescaler",
                    ),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Etps",
                    ),
                },
                Field {
                    name: "ece",
                    description: Some(
                        "External clock mode 2 enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etp",
                    description: Some(
                        "External trigger polarity",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Etp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "SrAdv",
            extends: Some(
                "SR_GP",
            ),
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccif",
                    description: Some(
                        "Capture/compare 1 interrupt flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comif",
                    description: Some(
                        "COM interrupt flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tif",
                    description: Some(
                        "Trigger interrupt flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bif",
                    description: Some(
                        "Break interrupt flag",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccof",
                    description: Some(
                        "Capture/Compare 1 overcapture flag",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SrBasic",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SrGp",
            extends: Some(
                "SR_BASIC",
            ),
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uif",
                    description: Some(
                        "Update interrupt flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccif",
                    description: Some(
                        "Capture/compare 1 interrupt flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "comif",
                    description: Some(
                        "COM interrupt flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tif",
                    description: Some(
                        "Trigger interrupt flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bif",
                    description: Some(
                        "Break interrupt flag",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccof",
                    description: Some(
                        "Capture/Compare 1 overcapture flag",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ccds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONCOMPARE",
                    description: Some(
                        "CCx DMA request sent when CCx event occurs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ONUPDATE",
                    description: Some(
                        "CCx DMA request sent when update event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CcmrInputCcs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TI4",
                    description: Some(
                        "CCx channel is configured as input, normal mapping: ICx mapped to TIx",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TI3",
                    description: Some(
                        "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TRC",
                    description: Some(
                        "CCx channel is configured as input, ICx is mapped on TRC",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcmrOutputCcs",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "CCx channel is configured as output",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ckd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "t_DTS = t_CK_INT",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "t_DTS = 2 × t_CK_INT",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "t_DTS = 4 × t_CK_INT",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Cms",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "EDGEALIGNED",
                    description: Some(
                        "The counter counts up or down depending on the direction bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CENTERALIGNED1",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CENTERALIGNED2",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CENTERALIGNED3",
                    description: Some(
                        "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UP",
                    description: Some(
                        "Counter used as upcounter",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "Counter used as downcounter",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Etf",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NOFILTER",
                    description: Some(
                        "No filter, sampling is done at fDTS",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FCK_INT_N2",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FCK_INT_N4",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FCK_INT_N8",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N6",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=6",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N8",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N6",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N8",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N6",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=6",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N8",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N5",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=5",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N6",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=6",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N8",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=8",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N5",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=5",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N6",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=6",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N8",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=8",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Etp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTINVERTED",
                    description: Some(
                        "ETR is noninverted, active at high level or rising edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: Some(
                        "ETR is inverted, active at low level or falling edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Etps",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Prescaler OFF",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "ETRP frequency divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "ETRP frequency divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "ETRP frequency divided by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Icf",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NOFILTER",
                    description: Some(
                        "No filter, sampling is done at fDTS",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FCK_INT_N2",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FCK_INT_N4",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FCK_INT_N8",
                    description: Some(
                        "fSAMPLING=fCK_INT, N=8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N6",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=6",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FDTS_DIV2_N8",
                    description: Some(
                        "fSAMPLING=fDTS/2, N=8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N6",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "FDTS_DIV4_N8",
                    description: Some(
                        "fSAMPLING=fDTS/4, N=8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N6",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=6",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "FDTS_DIV8_N8",
                    description: Some(
                        "fSAMPLING=fDTS/8, N=8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N5",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=5",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N6",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=6",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "FDTS_DIV16_N8",
                    description: Some(
                        "fSAMPLING=fDTS/16, N=8",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N5",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=5",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N6",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=6",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "FDTS_DIV32_N8",
                    description: Some(
                        "fSAMPLING=fDTS/32, N=8",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Mms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "The UG bit from the TIMx_EGR register is used as trigger output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "The counter enable signal, CNT_EN, is used as trigger output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UPDATE",
                    description: Some(
                        "The update event is selected as trigger output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "COMPAREPULSE",
                    description: Some(
                        "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "COMPAREOC1",
                    description: Some(
                        "OC1REF signal is used as trigger output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "COMPAREOC2",
                    description: Some(
                        "OC2REF signal is used as trigger output",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "COMPAREOC3",
                    description: Some(
                        "OC3REF signal is used as trigger output",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "COMPAREOC4",
                    description: Some(
                        "OC4REF signal is used as trigger output",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Msm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOSYNC",
                    description: Some(
                        "No action",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYNC",
                    description: Some(
                        "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ocm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "FROZEN",
                    description: Some(
                        "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVEONMATCH",
                    description: Some(
                        "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "INACTIVEONMATCH",
                    description: Some(
                        "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TOGGLE",
                    description: Some(
                        "OCyREF toggles when TIMx_CNT=TIMx_CCRy",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "FORCEINACTIVE",
                    description: Some(
                        "OCyREF is forced low",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "FORCEACTIVE",
                    description: Some(
                        "OCyREF is forced high",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "PWMMODE1",
                    description: Some(
                        "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PWMMODE2",
                    description: Some(
                        "Inversely to PwmMode1",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Ossi",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "When inactive, OC/OCN outputs are disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLELEVEL",
                    description: Some(
                        "When inactive, OC/OCN outputs are forced to idle level",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ossr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "When inactive, OC/OCN outputs are disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLELEVEL",
                    description: Some(
                        "When inactive, OC/OCN outputs are enabled with their inactive level",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENCODER_MODE_1",
                    description: Some(
                        "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ENCODER_MODE_2",
                    description: Some(
                        "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ENCODER_MODE_3",
                    description: Some(
                        "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RESET_MODE",
                    description: Some(
                        "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "GATED_MODE",
                    description: Some(
                        "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TRIGGER_MODE",
                    description: Some(
                        "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "EXT_CLOCK_MODE",
                    description: Some(
                        "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Tis",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "The TIMx_CH1 pin is connected to TI1 input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "XOR",
                    description: Some(
                        "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ts",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ITR0",
                    description: Some(
                        "Internal Trigger 0 (ITR0)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ITR1",
                    description: Some(
                        "Internal Trigger 1 (ITR1)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ITR2",
                    description: Some(
                        "Internal Trigger 2 (ITR2)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ITR3",
                    description: Some(
                        "Internal Trigger 3 (ITR3)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TI1F_ED",
                    description: Some(
                        "TI1 Edge Detector (TI1F_ED)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TI1FP1",
                    description: Some(
                        "Filtered Timer Input 1 (TI1FP1)",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TI2FP2",
                    description: Some(
                        "Filtered Timer Input 2 (TI2FP2)",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "ETRF",
                    description: Some(
                        "External Trigger input (ETRF)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Urs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANYEVENT",
                    description: Some(
                        "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "COUNTERONLY",
                    description: Some(
                        "Only counter overflow/underflow generates an update interrupt or DMA request",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                