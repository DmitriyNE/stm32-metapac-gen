include!("../metadata_0572.rs");
            pub static METADATA: Metadata = Metadata {
                name: "STM32L152VC-A",
                family: "STM32L1",
                line: "STM32L151/152",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 134217728,
        size: 196608,
        settings: Some(
            FlashSettings {
                erase_size: 256,
                write_size: 4,
                erase_value: 0,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_2",
        kind: MemoryRegionKind::Flash,
        address: 134414336,
        size: 65536,
        settings: Some(
            FlashSettings {
                erase_size: 256,
                write_size: 4,
                erase_value: 0,
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