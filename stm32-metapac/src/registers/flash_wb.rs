
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "Flash",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "Access control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "Flash key register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "Option byte key register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Flash control register",
                    ),
                    array: None,
                    byte_offset: 20,
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
                    name: "eccr",
                    description: Some(
                        "Flash ECC register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optr",
                    description: Some(
                        "Flash option register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcrop1asr",
                    description: Some(
                        "Flash Bank 1 PCROP Start address zone A register",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcrop1asr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcrop1aer",
                    description: Some(
                        "Flash Bank 1 PCROP End address zone A register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcrop1aer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1ar",
                    description: Some(
                        "Flash Bank 1 WRP area A address register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp1ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1br",
                    description: Some(
                        "Flash Bank 1 WRP area B address register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp1br",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcrop1bsr",
                    description: Some(
                        "Flash Bank 1 PCROP Start address area B register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcrop1bsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcrop1ber",
                    description: Some(
                        "Flash Bank 1 PCROP End address area B register",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcrop1ber",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipccbr",
                    description: Some(
                        "IPCC mailbox data buffer address register",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipccbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2acr",
                    description: Some(
                        "CPU2 cortex M0 access control register",
                    ),
                    array: None,
                    byte_offset: 92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2sr",
                    description: Some(
                        "CPU2 cortex M0 status register",
                    ),
                    array: None,
                    byte_offset: 96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c2cr",
                    description: Some(
                        "CPU2 cortex M0 control register",
                    ),
                    array: None,
                    byte_offset: 100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sfr",
                    description: Some(
                        "Secure flash start address register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "srrvr",
                    description: Some(
                        "Secure SRAM2 start address and cortex M0 reset vector register",
                    ),
                    array: None,
                    byte_offset: 132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Srrvr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some(
                "Access control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "Latency",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prften",
                    description: Some(
                        "Prefetch enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icen",
                    description: Some(
                        "Instruction cache enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcen",
                    description: Some(
                        "Data cache enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icrst",
                    description: Some(
                        "Instruction cache reset",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrst",
                    description: Some(
                        "Data cache reset",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pes",
                    description: Some(
                        "CPU1 CortexM4 program erase suspend request",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "empty",
                    description: Some(
                        "Flash User area empty",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2acr",
            extends: None,
            description: Some(
                "CPU2 cortex M0 access control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prften",
                    description: Some(
                        "CPU2 cortex M0 prefetch enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icen",
                    description: Some(
                        "CPU2 cortex M0 instruction cache enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icrst",
                    description: Some(
                        "CPU2 cortex M0 instruction cache reset",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pes",
                    description: Some(
                        "CPU2 cortex M0 program erase suspend request",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2cr",
            extends: None,
            description: Some(
                "CPU2 cortex M0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Programming",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "per",
                    description: Some(
                        "Page erase",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mer",
                    description: Some(
                        "Masse erase",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pnb",
                    description: Some(
                        "Page Number selection",
                    ),
                    bit_offset: 3,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strt",
                    description: Some(
                        "Start",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fstpg",
                    description: Some(
                        "Fast programming",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some(
                        "End of operation interrupt enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errie",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderrie",
                    description: Some(
                        "PCROP read error interrupt enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2sr",
            extends: None,
            description: Some(
                "CPU2 cortex M0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some(
                        "End of operation",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "operr",
                    description: Some(
                        "Operation error",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "progerr",
                    description: Some(
                        "Programming error",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrperr",
                    description: Some(
                        "write protection error",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgaerr",
                    description: Some(
                        "Programming alignment error",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sizerr",
                    description: Some(
                        "Size error",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgserr",
                    description: Some(
                        "Programming sequence error",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "misserr",
                    description: Some(
                        "Fast programming data miss error",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fasterr",
                    description: Some(
                        "Fast programming error",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderr",
                    description: Some(
                        "PCROP read error",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bsy",
                    description: Some(
                        "Busy",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfgbsy",
                    description: Some(
                        "Programming or erase configuration busy",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pesd",
                    description: Some(
                        "Programming or erase operation suspended",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Flash control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Programming",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "per",
                    description: Some(
                        "Page erase",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mer",
                    description: Some(
                        "This bit triggers the mass erase (all user pages) when set",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pnb",
                    description: Some(
                        "Page number selection",
                    ),
                    bit_offset: 3,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strt",
                    description: Some(
                        "Start",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optstrt",
                    description: Some(
                        "Options modification start",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fstpg",
                    description: Some(
                        "Fast programming",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some(
                        "End of operation interrupt enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errie",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderrie",
                    description: Some(
                        "PCROP read error interrupt enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "obl_launch",
                    description: Some(
                        "Force the option byte loading",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optlock",
                    description: Some(
                        "Options Lock",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "FLASH_CR Lock",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccr",
            extends: None,
            description: Some(
                "Flash ECC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC fail address",
                    ),
                    bit_offset: 0,
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_ecc",
                    description: Some(
                        "System Flash ECC fail",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ecccie",
                    description: Some(
                        "ECC correction interrupt enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpuid",
                    description: Some(
                        "CPU identification",
                    ),
                    bit_offset: 26,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccc",
                    description: Some(
                        "ECC correction",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccd",
                    description: Some(
                        "ECC detection",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipccbr",
            extends: None,
            description: Some(
                "IPCC mailbox data buffer address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipccdba",
                    description: Some(
                        "PCC mailbox data buffer base address",
                    ),
                    bit_offset: 0,
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Optr",
            extends: None,
            description: Some(
                "Flash option register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdp",
                    description: Some(
                        "Read protection level",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ese",
                    description: Some(
                        "Security enabled",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bor_lev",
                    description: Some(
                        "BOR reset Level",
                    ),
                    bit_offset: 9,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_stop",
                    description: Some(
                        "nRST_STOP",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_stdby",
                    description: Some(
                        "nRST_STDBY",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_shdw",
                    description: Some(
                        "nRST_SHDW",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idwg_sw",
                    description: Some(
                        "Independent watchdog selection",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stop",
                    description: Some(
                        "Independent watchdog counter freeze in Stop mode",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stdby",
                    description: Some(
                        "Independent watchdog counter freeze in Standby mode",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdg_sw",
                    description: Some(
                        "Window watchdog selection",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot1",
                    description: Some(
                        "Boot configuration",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2_pe",
                    description: Some(
                        "SRAM2 parity check enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2_rst",
                    description: Some(
                        "SRAM2 Erase when system reset",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_swboot0",
                    description: Some(
                        "Software Boot0",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot0",
                    description: Some(
                        "nBoot0 option bit",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "agc_trim",
                    description: Some(
                        "Radio Automatic Gain Control Trimming",
                    ),
                    bit_offset: 29,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcrop1aer",
            extends: None,
            description: Some(
                "Flash Bank 1 PCROP End address zone A register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcrop1a_end",
                    description: Some(
                        "Bank 1 PCROP area end offset",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcrop_rdp",
                    description: Some(
                        "PCROP area preserved when RDP level decreased",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcrop1asr",
            extends: None,
            description: Some(
                "Flash Bank 1 PCROP Start address zone A register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcrop1a_strt",
                    description: Some(
                        "Bank 1 PCROPQ area start offset",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcrop1ber",
            extends: None,
            description: Some(
                "Flash Bank 1 PCROP End address area B register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcrop1b_end",
                    description: Some(
                        "Bank 1 PCROP area end area B offset",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcrop1bsr",
            extends: None,
            description: Some(
                "Flash Bank 1 PCROP Start address area B register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcrop1b_strt",
                    description: Some(
                        "Bank 1 PCROP area B start offset",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sfr",
            extends: None,
            description: Some(
                "Secure flash start address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sfsa",
                    description: Some(
                        "Secure flash start address",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fsd",
                    description: Some(
                        "Flash security disable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dds",
                    description: Some(
                        "Disable Cortex M0 debug access",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some(
                        "End of operation",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "operr",
                    description: Some(
                        "Operation error",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "progerr",
                    description: Some(
                        "Programming error",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrperr",
                    description: Some(
                        "Write protected error",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgaerr",
                    description: Some(
                        "Programming alignment error",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sizerr",
                    description: Some(
                        "Size error",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgserr",
                    description: Some(
                        "Programming sequence error",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "miserr",
                    description: Some(
                        "Fast programming data miss error",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fasterr",
                    description: Some(
                        "Fast programming error",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optnv",
                    description: Some(
                        "User Option OPTVAL indication",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderr",
                    description: Some(
                        "PCROP read error",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optverr",
                    description: Some(
                        "Option validity error",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bsy",
                    description: Some(
                        "Busy",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfgbsy",
                    description: Some(
                        "Programming or erase configuration busy",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pesd",
                    description: Some(
                        "Programming or erase operation suspended",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Srrvr",
            extends: None,
            description: Some(
                "Secure SRAM2 start address and cortex M0 reset vector register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbrv",
                    description: Some(
                        "cortex M0 access control register",
                    ),
                    bit_offset: 0,
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbrsa",
                    description: Some(
                        "Secure backup SRAM2a start address",
                    ),
                    bit_offset: 18,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brsd",
                    description: Some(
                        "backup SRAM2a security disable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snbrsa",
                    description: Some(
                        "Secure non backup SRAM2a start address",
                    ),
                    bit_offset: 25,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbrsd",
                    description: Some(
                        "non-backup SRAM2b security disable",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2opt",
                    description: Some(
                        "CPU2 cortex M0 boot reset vector memory selection",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp1ar",
            extends: None,
            description: Some(
                "Flash Bank 1 WRP area A address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1a_strt",
                    description: Some(
                        "Bank 1 WRP first area A start offset",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1a_end",
                    description: Some(
                        "Bank 1 WRP first area A end offset",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp1br",
            extends: None,
            description: Some(
                "Flash Bank 1 WRP area B address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1b_end",
                    description: Some(
                        "Bank 1 WRP second area B start offset",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1b_strt",
                    description: Some(
                        "Bank 1 WRP second area B end offset",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                