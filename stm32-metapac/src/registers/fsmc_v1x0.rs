
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fsmc",
            extends: None,
            description: Some(
                "Flexible static memory controller",
            ),
            items: &[
                BlockItem {
                    name: "bcr",
                    description: Some(
                        "SRAM/NOR-Flash chip-select control register 1-4",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btr",
                    description: Some(
                        "SRAM/NOR-Flash chip-select timing register 1-4",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Btr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bwtr",
                    description: Some(
                        "SRAM/NOR-Flash write timing registers 1-4",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bwtr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bcr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash chip-select control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mbken",
                    description: Some(
                        "Memory bank enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "muxen",
                    description: Some(
                        "Address/data multiplexing enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtyp",
                    description: Some(
                        "Memory type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mtyp",
                    ),
                },
                Field {
                    name: "mwid",
                    description: Some(
                        "Memory data bus width",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mwid",
                    ),
                },
                Field {
                    name: "faccen",
                    description: Some(
                        "Flash access enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bursten",
                    description: Some(
                        "Burst enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waitpol",
                    description: Some(
                        "Wait signal polarity bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitpol",
                    ),
                },
                Field {
                    name: "wrapmod",
                    description: Some(
                        "WRAPMOD",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waitcfg",
                    description: Some(
                        "Wait timing configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Waitcfg",
                    ),
                },
                Field {
                    name: "wren",
                    description: Some(
                        "Write enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "waiten",
                    description: Some(
                        "Wait enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extmod",
                    description: Some(
                        "Extended mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asyncwait",
                    description: Some(
                        "Wait signal during asynchronous transfers",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpsize",
                    description: Some(
                        "CRAM page size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cpsize",
                    ),
                },
                Field {
                    name: "cburstrw",
                    description: Some(
                        "Write burst enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Btr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash chip-select timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addset",
                    description: Some(
                        "Address setup phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addhld",
                    description: Some(
                        "Address-hold phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datast",
                    description: Some(
                        "Data-phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busturn",
                    description: Some(
                        "Bus turnaround phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clkdiv",
                    description: Some(
                        "Clock divide ratio (for FMC_CLK signal)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datlat",
                    description: Some(
                        "Data latency for synchronous memory",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "accmod",
                    description: Some(
                        "Access mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Accmod",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bwtr",
            extends: None,
            description: Some(
                "SRAM/NOR-Flash write timing registers",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addset",
                    description: Some(
                        "Address setup phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addhld",
                    description: Some(
                        "Address-hold phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datast",
                    description: Some(
                        "Data-phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busturn",
                    description: Some(
                        "Bus turnaround phase duration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "accmod",
                    description: Some(
                        "Access mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Accmod",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Accmod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "A",
                    description: Some(
                        "Access mode A",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B",
                    description: Some(
                        "Access mode B",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "C",
                    description: Some(
                        "Access mode C",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "D",
                    description: Some(
                        "Access mode D",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cpsize",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_BURST_SPLIT",
                    description: Some(
                        "No burst split when crossing page boundary",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYTES128",
                    description: Some(
                        "128 bytes CRAM page size",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTES256",
                    description: Some(
                        "256 bytes CRAM page size",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BYTES512",
                    description: Some(
                        "512 bytes CRAM page size",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BYTES1024",
                    description: Some(
                        "1024 bytes CRAM page size",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Mtyp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "SRAM memory type",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PSRAM",
                    description: Some(
                        "PSRAM (CRAM) memory type",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FLASH",
                    description: Some(
                        "NOR Flash/OneNAND Flash",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mwid",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "Memory data bus width 8 bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "Memory data bus width 16 bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "Memory data bus width 32 bits",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Waitcfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BEFORE_WAIT_STATE",
                    description: Some(
                        "NWAIT signal is active one data cycle before wait state",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DURING_WAIT_STATE",
                    description: Some(
                        "NWAIT signal is active during wait state",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Waitpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE_LOW",
                    description: Some(
                        "NWAIT active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE_HIGH",
                    description: Some(
                        "NWAIT active high",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                