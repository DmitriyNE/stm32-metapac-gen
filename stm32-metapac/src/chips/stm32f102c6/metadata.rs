include!("../metadata_0083.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32F102C6",
                family: "STM32F1",
                line: "STM32F102",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 32768,
        settings: Some(
            FlashSettings {
                erase_size: 1024,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 536870912,
        size: 6144,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };