
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
                    name: "seccfgr",
                    description: Some(
                        "SYSCFG secure configuration\r register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpuimr",
                    description: Some(
                        "FPU interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpuimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnslckr",
                    description: Some(
                        "SYSCFG CPU non-secure lock\r register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnslckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cslockr",
                    description: Some(
                        "SYSCFG CPU secure lock\r register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cslockr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mesr",
                    description: Some(
                        "memory erase status register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mesr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccsr",
                    description: Some(
                        "compensation cell control/status register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccvr",
                    description: Some(
                        "compensation cell value register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccr",
                    description: Some(
                        "compensation cell code register",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsscmdr",
                    description: Some(
                        "RSS command register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsscmdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ucpdr",
                    description: Some(
                        "USB Type C and Power Delivery register",
                    ),
                    array: None,
                    byte_offset: 112,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ucpdr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cccr",
            extends: None,
            description: Some(
                "compensation cell code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncc1",
                    description: Some(
                        "NCC1",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcc1",
                    description: Some(
                        "PCC1",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ncc2",
                    description: Some(
                        "NCC2",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcc2",
                    description: Some(
                        "PCC2",
                    ),
                    bit_offset: 12,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cccsr",
            extends: None,
            description: Some(
                "compensation cell control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "EN1",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cs1",
                    description: Some(
                        "CS1",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en2",
                    description: Some(
                        "EN2",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cs2",
                    description: Some(
                        "CS2",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdy1",
                    description: Some(
                        "RDY1",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdy2",
                    description: Some(
                        "RDY2",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccvr",
            extends: None,
            description: Some(
                "compensation cell value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncv1",
                    description: Some(
                        "NCV1",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcv1",
                    description: Some(
                        "PCV1",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ncv2",
                    description: Some(
                        "NCV2",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcv2",
                    description: Some(
                        "PCV2",
                    ),
                    bit_offset: 12,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boosten",
                    description: Some(
                        "I/O analog switch voltage booster\r enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "anaswvdd",
                    description: Some(
                        "GPIO analog switch control voltage\r selection",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb6_fmp",
                    description: Some(
                        "PB6_FMP",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb7_fmp",
                    description: Some(
                        "PB7_FMP",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb8_fmp",
                    description: Some(
                        "PB8_FMP",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb9_fmp",
                    description: Some(
                        "PB9_FMP",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cll",
                    description: Some(
                        "LOCKUP (hardfault) output enable\r bit",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spl",
                    description: Some(
                        "SRAM ECC lock bit",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdl",
                    description: Some(
                        "PVD lock enable bit",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccl",
                    description: Some(
                        "ECC Lock",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cnslckr",
            extends: None,
            description: Some(
                "SYSCFG CPU non-secure lock\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locknsvtor",
                    description: Some(
                        "VTOR_NS register lock",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "locknsmpu",
                    description: Some(
                        "Non-secure MPU registers\r lock",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cslockr",
            extends: None,
            description: Some(
                "SYSCFG CPU secure lock\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locksvtaircr",
                    description: Some(
                        "LOCKSVTAIRCR",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "locksmpu",
                    description: Some(
                        "LOCKSMPU",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "locksau",
                    description: Some(
                        "LOCKSAU",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fpuimr",
            extends: None,
            description: Some(
                "FPU interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpu_ie",
                    description: Some(
                        "Floating point unit interrupts enable\r bits",
                    ),
                    bit_offset: 0,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mesr",
            extends: None,
            description: Some(
                "memory erase status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mclr",
                    description: Some(
                        "MCLR",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipmee",
                    description: Some(
                        "IPMEE",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsscmdr",
            extends: None,
            description: Some(
                "RSS command register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsscmd",
                    description: Some(
                        "RSS commands",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "SYSCFG secure configuration\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsec",
                    description: Some(
                        "SYSCFG clock control\r security",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "classbsec",
                    description: Some(
                        "CLASSBSEC",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpusec",
                    description: Some(
                        "FPUSEC",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ucpdr",
            extends: None,
            description: Some(
                "USB Type C and Power Delivery register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cc1enrxfilter",
                    description: Some(
                        "CC1ENRXFILTER",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cc2enrxfilter",
                    description: Some(
                        "CC2ENRXFILTER",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                