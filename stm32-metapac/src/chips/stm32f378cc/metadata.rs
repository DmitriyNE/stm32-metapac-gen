include!("../metadata_0149.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32F378CC",
                family: "STM32F3",
                line: "STM32F3x8",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 262144,
        settings: Some(
            FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 536870912,
        size: 32768,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };