include!("../metadata_0025.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32F042G4",
                family: "STM32F0",
                line: "STM32F0x2",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 16384,
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
                nvic_priority_bits: Some(2),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };