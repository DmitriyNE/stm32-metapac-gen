include!("../metadata_0446.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32H755ZI",
                family: "STM32H7",
                line: "STM32H745/755",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_2",
        kind: MemoryRegionKind::Flash,
        address: 135266304,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "D1_AXIFLASH",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 0,
        settings: None,
    },
    MemoryRegion {
        name: "D1_AXIICP",
        kind: MemoryRegionKind::Flash,
        address: 535822336,
        size: 0,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 603979776,
        size: 524288,
        settings: None,
    },
    MemoryRegion {
        name: "D1_ITCMRAM",
        kind: MemoryRegionKind::Ram,
        address: 0,
        size: 0,
        settings: None,
    },
    MemoryRegion {
        name: "D1_DTCMRAM",
        kind: MemoryRegionKind::Ram,
        address: 536870912,
        size: 0,
        settings: None,
    },
    MemoryRegion {
        name: "D2_AXISRAM",
        kind: MemoryRegionKind::Ram,
        address: 268435456,
        size: 0,
        settings: None,
    },
    MemoryRegion {
        name: "D3_BKPSRAM",
        kind: MemoryRegionKind::Ram,
        address: 947912704,
        size: 0,
        settings: None,
    },
    MemoryRegion {
        name: "D3_SRAM",
        kind: MemoryRegionKind::Ram,
        address: 939524096,
        size: 0,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };