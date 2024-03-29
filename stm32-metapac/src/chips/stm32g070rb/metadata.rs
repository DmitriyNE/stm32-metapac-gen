include!("../metadata_0291.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32G070RB",
                family: "STM32G0",
                line: "STM32G0x0 Value line",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 131072,
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
                nvic_priority_bits: Some(2),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };