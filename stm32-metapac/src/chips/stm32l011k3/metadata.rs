include!("../metadata_0497.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32L011K3",
    family: "STM32L0",
    line: "STM32L0x1",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 8192,
            settings: Some(FlashSettings {
                erase_size: 128,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536870912,
            size: 2048,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
