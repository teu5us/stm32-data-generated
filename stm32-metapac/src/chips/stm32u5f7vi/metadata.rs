include!("../metadata_0792.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32U5F7VI",
    family: "STM32U5",
    line: "STM32U5F7/5G7",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 1048576,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8200000,
            size: 1048576,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 786432,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x200c0000,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM3",
            kind: MemoryRegionKind::Ram,
            address: 0x200d0000,
            size: 851968,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM5",
            kind: MemoryRegionKind::Ram,
            address: 0x201a0000,
            size: 851968,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM6",
            kind: MemoryRegionKind::Ram,
            address: 0x20270000,
            size: 524288,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
