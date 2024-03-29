include!("../metadata_0082.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32F101ZF",
                family: "STM32F1",
                line: "STM32F101",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 524288,
        settings: Some(
            FlashSettings {
                erase_size: 2048,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_2",
        kind: MemoryRegionKind::Flash,
        address: 134742016,
        size: 262144,
        settings: Some(
            FlashSettings {
                erase_size: 2048,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 536870912,
        size: 81920,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };