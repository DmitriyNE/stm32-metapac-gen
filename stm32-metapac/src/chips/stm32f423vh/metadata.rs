include!("../metadata_0188.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32F423VH",
                family: "STM32F4",
                line: "STM32F413/423",
                memory: &[
    MemoryRegion {
        name: "BANK_1_REGION_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 65536,
        settings: Some(
            FlashSettings {
                erase_size: 16384,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_1_REGION_2",
        kind: MemoryRegionKind::Flash,
        address: 134283264,
        size: 65536,
        settings: Some(
            FlashSettings {
                erase_size: 65536,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_1_REGION_3",
        kind: MemoryRegionKind::Flash,
        address: 134348800,
        size: 1441792,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "OTP",
        kind: MemoryRegionKind::Flash,
        address: 536836096,
        size: 528,
        settings: Some(
            FlashSettings {
                erase_size: 528,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 536870912,
        size: 327680,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM2",
        kind: MemoryRegionKind::Ram,
        address: 537133056,
        size: 0,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };