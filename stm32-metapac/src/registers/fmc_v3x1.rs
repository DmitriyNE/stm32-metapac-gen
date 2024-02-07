
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fmc",
            extends: None,
            description: Some(
                "Flexible memory controller",
            ),
            items: &[
                BlockItem {
                    name: "bcr1",
                    description: Some(
                        "SRAM/NOR-Flash chip-select control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btr",
                    description: Some(
                        "SRAM/NOR-Flash chip-select timing register 1-4",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Btr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bcr",
                    description: Some(
                        "SRAM/NOR-Flash chip-select control register 2-4",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcr",
                    description: Some(
                        "PC Card/NAND Flash control register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "FIFO status and interrupt register",
                    ),
                    array: None,
                    byte_offset: 132,
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
                    name: "pmem",
                    description: Some(
                        "Common memory space timing register",
                    ),
                    array: None,
                    byte_offset: 136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "patt",
                    description: Some(
                        "Attribute memory space timing register",
                    ),
                    array: None,
                    byte_offset: 140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Patt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr",
                    description: Some(
                        "ECC result register",
                    ),
                    array: None,
                    byte_offset: 148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bwtr",
                    description: Some(
                        "SRAM/NOR-Flash write timing registers 1-4",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bwtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdcr",
                    description: Some(
                        "SDRAM Control Register 1-2",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 320,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdtr",
                    description: Some(
                        "SDRAM Timing register 1-2",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdcmr",
                    description: Some(
                        "SDRAM Command Mode register",
                    ),
                    array: None,
                    byte_offset: 336,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdcmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdrtr",
                    description: Some(
                        "SDRAM Refresh Timer register",
                    ),
                    array: None,
                    byte_offset: 340,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdrtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdsr",
                    description: Some(
                        "SDRAM Status register",
                    ),
                    array: None,
                    byte_offset: 344,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bcr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash chip-select control register 2-4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mbken",
                    description: Some(
                        "Memory bank enable bit",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "muxen",
                    description: Some(
                        "Address/data multiplexing enable bit",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtyp",
                    description: Some(
                        "Memory type",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mtyp",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "faccen",
                    description: Some(
                        "Flash access enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bursten",
                    description: Some(
                        "Burst enable bit",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waitpol",
                    description: Some(
                        "Wait signal polarity bit",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitpol",
                    ),
                },
                Field {
                    name: "waitcfg",
                    description: Some(
                        "Wait timing configuration",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitcfg",
                    ),
                },
                Field {
                    name: "wren",
                    description: Some(
                        "Write enable bit",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waiten",
                    description: Some(
                        "Wait enable bit",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extmod",
                    description: Some(
                        "Extended mode enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asyncwait",
                    description: Some(
                        "Wait signal during asynchronous transfers",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpsize",
                    description: Some(
                        "CRAM page size",
                    ),
                    bit_offset: 16,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cpsize",
                    ),
                },
                Field {
                    name: "cburstrw",
                    description: Some(
                        "Write burst enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bcr1",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash chip-select control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mbken",
                    description: Some(
                        "Memory bank enable bit",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "muxen",
                    description: Some(
                        "Address/data multiplexing enable bit",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtyp",
                    description: Some(
                        "Memory type",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mtyp",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "faccen",
                    description: Some(
                        "Flash access enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bursten",
                    description: Some(
                        "Burst enable bit",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waitpol",
                    description: Some(
                        "Wait signal polarity bit",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitpol",
                    ),
                },
                Field {
                    name: "waitcfg",
                    description: Some(
                        "Wait timing configuration",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitcfg",
                    ),
                },
                Field {
                    name: "wren",
                    description: Some(
                        "Write enable bit",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waiten",
                    description: Some(
                        "Wait enable bit",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extmod",
                    description: Some(
                        "Extended mode enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asyncwait",
                    description: Some(
                        "Wait signal during asynchronous transfers",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpsize",
                    description: Some(
                        "CRAM page size",
                    ),
                    bit_offset: 16,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cpsize",
                    ),
                },
                Field {
                    name: "cburstrw",
                    description: Some(
                        "Write burst enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cclken",
                    description: Some(
                        "Continuous clock enable",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wfdis",
                    description: Some(
                        "Write FIFO disable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmap",
                    description: Some(
                        "FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register.",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmcen",
                    description: Some(
                        "FMC controller enable",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Btr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash chip-select timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addset",
                    description: Some(
                        "Address setup phase duration",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addhld",
                    description: Some(
                        "Address-hold phase duration",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datast",
                    description: Some(
                        "Data-phase duration",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busturn",
                    description: Some(
                        "Bus turnaround phase duration",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clkdiv",
                    description: Some(
                        "Clock divide ratio (for FMC_CLK signal)",
                    ),
                    bit_offset: 20,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datlat",
                    description: Some(
                        "Data latency for synchronous memory",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "accmod",
                    description: Some(
                        "Access mode",
                    ),
                    bit_offset: 28,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Accmod",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bwtr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash write timing registers",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addset",
                    description: Some(
                        "Address setup phase duration",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addhld",
                    description: Some(
                        "Address-hold phase duration",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datast",
                    description: Some(
                        "Data-phase duration",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busturn",
                    description: Some(
                        "Bus turnaround phase duration",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "accmod",
                    description: Some(
                        "Access mode",
                    ),
                    bit_offset: 28,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Accmod",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Eccr",
            extends: None,
            description: Some(
                "ECC result register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecc",
                    description: Some(
                        "ECC computation result value",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Patt",
            extends: None,
            description: Some(
                "Attribute memory space timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "attset",
                    description: Some(
                        "Attribute memory setup time",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "attwait",
                    description: Some(
                        "Attribute memory wait time",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atthold",
                    description: Some(
                        "Attribute memory hold time",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atthiz",
                    description: Some(
                        "Attribute memory data bus Hi-Z time",
                    ),
                    bit_offset: 24,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcr",
            extends: None,
            description: Some(
                "PC Card/NAND Flash control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwaiten",
                    description: Some(
                        "Wait feature enable bit",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pbken",
                    description: Some(
                        "NAND Flash memory bank enable bit",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwid",
                    description: Some(
                        "Data bus width",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pwid",
                    ),
                },
                Field {
                    name: "eccen",
                    description: Some(
                        "ECC computation logic enable bit",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tclr",
                    description: Some(
                        "CLE to RE delay",
                    ),
                    bit_offset: 9,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tar",
                    description: Some(
                        "ALE to RE delay",
                    ),
                    bit_offset: 13,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccps",
                    description: Some(
                        "ECC page size",
                    ),
                    bit_offset: 17,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Eccps",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pmem",
            extends: None,
            description: Some(
                "Common memory space timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "memset",
                    description: Some(
                        "Common memory x setup time",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "memwait",
                    description: Some(
                        "Common memory wait time",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "memhold",
                    description: Some(
                        "Common memory hold time",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "memhiz",
                    description: Some(
                        "Common memory x data bus Hi-Z time",
                    ),
                    bit_offset: 24,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sdcmr",
            extends: None,
            description: Some(
                "SDRAM Command Mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Command mode",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "ctb2",
                    description: Some(
                        "Command target bank 2",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ctb1",
                    description: Some(
                        "Command target bank 1",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nrfs",
                    description: Some(
                        "Number of Auto-refresh",
                    ),
                    bit_offset: 5,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrd",
                    description: Some(
                        "Mode Register definition",
                    ),
                    bit_offset: 9,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sdcr",
            extends: None,
            description: Some(
                "SDRAM Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nc",
                    description: Some(
                        "Number of column address bits",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Nc",
                    ),
                },
                Field {
                    name: "nr",
                    description: Some(
                        "Number of row address bits",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Nr",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "nb",
                    description: Some(
                        "Number of internal banks",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nb",
                    ),
                },
                Field {
                    name: "cas",
                    description: Some(
                        "CAS latency",
                    ),
                    bit_offset: 7,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cas",
                    ),
                },
                Field {
                    name: "wp",
                    description: Some(
                        "Write protection",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdclk",
                    description: Some(
                        "SDRAM clock configuration",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sdclk",
                    ),
                },
                Field {
                    name: "rburst",
                    description: Some(
                        "Burst read",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpipe",
                    description: Some(
                        "Read pipe",
                    ),
                    bit_offset: 13,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rpipe",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sdrtr",
            extends: None,
            description: Some(
                "SDRAM Refresh Timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cre",
                    description: Some(
                        "Clear Refresh error flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "count",
                    description: Some(
                        "Refresh Timer Count",
                    ),
                    bit_offset: 1,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reie",
                    description: Some(
                        "RES Interrupt Enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sdsr",
            extends: None,
            description: Some(
                "SDRAM Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "Refresh error flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "modes1",
                    description: Some(
                        "Status Mode for Bank 1",
                    ),
                    bit_offset: 1,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Modes",
                    ),
                },
                Field {
                    name: "modes2",
                    description: Some(
                        "Status Mode for Bank 2",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Modes",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sdtr",
            extends: None,
            description: Some(
                "SDRAM Timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmrd",
                    description: Some(
                        "Load Mode Register to Active",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txsr",
                    description: Some(
                        "Exit self-refresh delay",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tras",
                    description: Some(
                        "Self refresh time",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trc",
                    description: Some(
                        "Row cycle delay",
                    ),
                    bit_offset: 12,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twr",
                    description: Some(
                        "Recovery delay",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trp",
                    description: Some(
                        "Row precharge delay",
                    ),
                    bit_offset: 20,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trcd",
                    description: Some(
                        "Row to column delay",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "FIFO status and interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irs",
                    description: Some(
                        "Interrupt rising edge status",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ils",
                    description: Some(
                        "Interrupt high-level status",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ifs",
                    description: Some(
                        "Interrupt falling edge status",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iren",
                    description: Some(
                        "Interrupt rising edge detection enable bit",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilen",
                    description: Some(
                        "Interrupt high-level detection enable bit",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ifen",
                    description: Some(
                        "Interrupt falling edge detection enable bit",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fempt",
                    description: Some(
                        "FIFO empty status",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Accmod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "A",
                    description: Some(
                        "Access mode A",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B",
                    description: Some(
                        "Access mode B",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "C",
                    description: Some(
                        "Access mode C",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "D",
                    description: Some(
                        "Access mode D",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cas",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CLOCKS1",
                    description: Some(
                        "1 cycle",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CLOCKS2",
                    description: Some(
                        "2 cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CLOCKS3",
                    description: Some(
                        "3 cycles",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cpsize",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NOBURSTSPLIT",
                    description: Some(
                        "No burst split when crossing page boundary",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTES128",
                    description: Some(
                        "128 bytes CRAM page size",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTES256",
                    description: Some(
                        "256 bytes CRAM page size",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BYTES512",
                    description: Some(
                        "512 bytes CRAM page size",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BYTES1024",
                    description: Some(
                        "1024 bytes CRAM page size",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Eccps",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BYTES256",
                    description: Some(
                        "ECC page size 256 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTES512",
                    description: Some(
                        "ECC page size 512 bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTES1024",
                    description: Some(
                        "ECC page size 1024 bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BYTES2048",
                    description: Some(
                        "ECC page size 2048 bytes",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BYTES4096",
                    description: Some(
                        "ECC page size 4096 bytes",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BYTES8192",
                    description: Some(
                        "ECC page size 8192 bytes",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal Mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CLOCKCONFIGURATIONENABLE",
                    description: Some(
                        "Clock Configuration Enable",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PALL",
                    description: Some(
                        "PALL (All Bank Precharge) command",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUTOREFRESHCOMMAND",
                    description: Some(
                        "Auto-refresh command",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LOADMODEREGISTER",
                    description: Some(
                        "Load Mode Resgier",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SELFREFRESHCOMMAND",
                    description: Some(
                        "Self-refresh command",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "POWERDOWNCOMMAND",
                    description: Some(
                        "Power-down command",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Modes",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal Mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SELFREFRESH",
                    description: Some(
                        "Self-refresh mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "POWERDOWN",
                    description: Some(
                        "Power-down mode",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mtyp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "SRAM memory type",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PSRAM",
                    description: Some(
                        "PSRAM (CRAM) memory type",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FLASH",
                    description: Some(
                        "NOR Flash/OneNAND Flash",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mwid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "Memory data bus width 8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "Memory data bus width 16 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "Memory data bus width 32 bits",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Nb",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NB2",
                    description: Some(
                        "Two internal Banks",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NB4",
                    description: Some(
                        "Four internal Banks",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Nc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS9",
                    description: Some(
                        "9 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10 bits",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS11",
                    description: Some(
                        "11 bits",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Nr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS11",
                    description: Some(
                        "11 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS12",
                    description: Some(
                        "12 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS13",
                    description: Some(
                        "13 bits",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Pwid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "External memory device width 8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "External memory device width 16 bits",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rpipe",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NODELAY",
                    description: Some(
                        "No clock cycle delay",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CLOCKS1",
                    description: Some(
                        "One clock cycle delay",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CLOCKS2",
                    description: Some(
                        "Two clock cycles delay",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Sdclk",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "SDCLK clock disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "SDCLK period = 2 x HCLK period",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "SDCLK period = 3 x HCLK period",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Waitcfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BEFOREWAITSTATE",
                    description: Some(
                        "NWAIT signal is active one data cycle before wait state",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DURINGWAITSTATE",
                    description: Some(
                        "NWAIT signal is active during wait state",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Waitpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVELOW",
                    description: Some(
                        "NWAIT active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVEHIGH",
                    description: Some(
                        "NWAIT active high",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                