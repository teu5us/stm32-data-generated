
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dma2d",
            extends: None,
            description: Some(
                "DMA2D controller",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "Interrupt Status Register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ifcr",
                    description: Some(
                        "interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgmar",
                    description: Some(
                        "foreground memory address register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgor",
                    description: Some(
                        "foreground offset register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgmar",
                    description: Some(
                        "background memory address register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgor",
                    description: Some(
                        "background offset register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgpfccr",
                    description: Some(
                        "foreground PFC control register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgpfccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgcolr",
                    description: Some(
                        "foreground color register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgcolr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgpfccr",
                    description: Some(
                        "background PFC control register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgpfccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgcolr",
                    description: Some(
                        "background color register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgcolr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgcmar",
                    description: Some(
                        "foreground CLUT memory address register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgcmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgcmar",
                    description: Some(
                        "background CLUT memory address register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgcmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "opfccr",
                    description: Some(
                        "output PFC control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Opfccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ocolr",
                    description: Some(
                        "output color register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ocolr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "omar",
                    description: Some(
                        "output memory address register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Omar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oor",
                    description: Some(
                        "output offset register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Oor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nlr",
                    description: Some(
                        "number of line register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lwr",
                    description: Some(
                        "line watermark register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lwr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "amtcr",
                    description: Some(
                        "AHB master timer configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Amtcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fgclut",
                    description: Some(
                        "FGCLUT",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fgclut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgclut",
                    description: Some(
                        "BGCLUT",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgclut",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Amtcr",
            extends: None,
            description: Some(
                "AHB master timer configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable",
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
                    name: "dt",
                    description: Some(
                        "Dead Time",
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
            ],
        },
        FieldSet {
            name: "Bgclut",
            extends: None,
            description: Some(
                "BGCLUT",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "BLUE",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "green",
                    description: Some(
                        "GREEN",
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
                    name: "red",
                    description: Some(
                        "RED",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aplha",
                    description: Some(
                        "APLHA",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bgcmar",
            extends: None,
            description: Some(
                "background CLUT memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bgcolr",
            extends: None,
            description: Some(
                "background color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "green",
                    description: Some(
                        "Green Value",
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
                    name: "red",
                    description: Some(
                        "Red Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bgmar",
            extends: None,
            description: Some(
                "background memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bgor",
            extends: None,
            description: Some(
                "background offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bgpfccr",
            extends: None,
            description: Some(
                "background PFC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm",
                    description: Some(
                        "Color mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "BgpfccrCm",
                    ),
                },
                Field {
                    name: "ccm",
                    description: Some(
                        "CLUT Color mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BgpfccrCcm",
                    ),
                },
                Field {
                    name: "start",
                    description: Some(
                        "Start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BgpfccrStart",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "CLUT size",
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
                    name: "am",
                    description: Some(
                        "Alpha mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "BgpfccrAm",
                    ),
                },
                Field {
                    name: "alpha",
                    description: Some(
                        "Alpha value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CrStart",
                    ),
                },
                Field {
                    name: "susp",
                    description: Some(
                        "Suspend",
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
                    name: "abort",
                    description: Some(
                        "Abort",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Abort",
                    ),
                },
                Field {
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Transfer complete interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twie",
                    description: Some(
                        "Transfer watermark interrupt enable",
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
                    name: "caeie",
                    description: Some(
                        "CLUT access error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ctcie",
                    description: Some(
                        "CLUT transfer complete interrupt enable",
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
                    name: "ceie",
                    description: Some(
                        "Configuration Error Interrupt Enable",
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
                    name: "mode",
                    description: Some(
                        "DMA2D mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Fgclut",
            extends: None,
            description: Some(
                "FGCLUT",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "BLUE",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "green",
                    description: Some(
                        "GREEN",
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
                    name: "red",
                    description: Some(
                        "RED",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aplha",
                    description: Some(
                        "APLHA",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fgcmar",
            extends: None,
            description: Some(
                "foreground CLUT memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fgcolr",
            extends: None,
            description: Some(
                "foreground color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "green",
                    description: Some(
                        "Green Value",
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
                    name: "red",
                    description: Some(
                        "Red Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fgmar",
            extends: None,
            description: Some(
                "foreground memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fgor",
            extends: None,
            description: Some(
                "foreground offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fgpfccr",
            extends: None,
            description: Some(
                "foreground PFC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm",
                    description: Some(
                        "Color mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "FgpfccrCm",
                    ),
                },
                Field {
                    name: "ccm",
                    description: Some(
                        "CLUT color mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FgpfccrCcm",
                    ),
                },
                Field {
                    name: "start",
                    description: Some(
                        "Start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "FgpfccrStart",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "CLUT size",
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
                    name: "am",
                    description: Some(
                        "Alpha mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "FgpfccrAm",
                    ),
                },
                Field {
                    name: "alpha",
                    description: Some(
                        "Alpha value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ifcr",
            extends: None,
            description: Some(
                "interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cteif",
                    description: Some(
                        "Clear Transfer error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cteif",
                    ),
                },
                Field {
                    name: "ctcif",
                    description: Some(
                        "Clear transfer complete interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ctcif",
                    ),
                },
                Field {
                    name: "ctwif",
                    description: Some(
                        "Clear transfer watermark interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ctwif",
                    ),
                },
                Field {
                    name: "caecif",
                    description: Some(
                        "Clear CLUT access error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Caecif",
                    ),
                },
                Field {
                    name: "cctcif",
                    description: Some(
                        "Clear CLUT transfer complete interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cctcif",
                    ),
                },
                Field {
                    name: "cceif",
                    description: Some(
                        "Clear configuration error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cceif",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "Interrupt Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teif",
                    description: Some(
                        "Transfer error interrupt flag",
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
                    name: "tcif",
                    description: Some(
                        "Transfer complete interrupt flag",
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
                    name: "twif",
                    description: Some(
                        "Transfer watermark interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "caeif",
                    description: Some(
                        "CLUT access error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ctcif",
                    description: Some(
                        "CLUT transfer complete interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ceif",
                    description: Some(
                        "Configuration error interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lwr",
            extends: None,
            description: Some(
                "line watermark register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lw",
                    description: Some(
                        "Line watermark",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nlr",
            extends: None,
            description: Some(
                "number of line register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nl",
                    description: Some(
                        "Number of lines",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pl",
                    description: Some(
                        "Pixel per lines",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ocolr",
            extends: None,
            description: Some(
                "output color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "green",
                    description: Some(
                        "Green Value",
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
                    name: "red",
                    description: Some(
                        "Red Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aplha",
                    description: Some(
                        "Alpha Channel Value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Omar",
            extends: None,
            description: Some(
                "output memory address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ma",
                    description: Some(
                        "Memory Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Oor",
            extends: None,
            description: Some(
                "output offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "Line Offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Opfccr",
            extends: None,
            description: Some(
                "output PFC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm",
                    description: Some(
                        "Color mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "OpfccrCm",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Abort",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ABORT_REQUEST",
                    description: Some(
                        "Transfer abort requested",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BgpfccrAm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_MODIFY",
                    description: Some(
                        "No modification of alpha channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REPLACE",
                    description: Some(
                        "Replace with value in ALPHA[7:0]",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MULTIPLY",
                    description: Some(
                        "Multiply with value in ALPHA[7:0]",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "BgpfccrCcm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "CLUT color format ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "CLUT color format RGB888",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BgpfccrCm",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "Color mode ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "Color mode RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "Color mode RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "Color mode ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "Color mode ARGB4444",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "L8",
                    description: Some(
                        "Color mode L8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "AL44",
                    description: Some(
                        "Color mode AL44",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "AL88",
                    description: Some(
                        "Color mode AL88",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "L4",
                    description: Some(
                        "Color mode L4",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "A8",
                    description: Some(
                        "Color mode A8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "A4",
                    description: Some(
                        "Color mode A4",
                    ),
                    value: 10,
                },
            ],
        },
        Enum {
            name: "BgpfccrStart",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Start the automatic loading of the CLUT",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Caecif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the CAEIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cceif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the CEIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cctcif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the CTCIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CrStart",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Launch the DMA2D",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ctcif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the TCIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cteif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the TEIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ctwif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear the TWIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FgpfccrAm",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_MODIFY",
                    description: Some(
                        "No modification of alpha channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REPLACE",
                    description: Some(
                        "Replace with value in ALPHA[7:0]",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MULTIPLY",
                    description: Some(
                        "Multiply with value in ALPHA[7:0]",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "FgpfccrCcm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "CLUT color format ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "CLUT color format RGB888",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FgpfccrCm",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "Color mode ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "Color mode RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "Color mode RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "Color mode ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "Color mode ARGB4444",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "L8",
                    description: Some(
                        "Color mode L8",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "AL44",
                    description: Some(
                        "Color mode AL44",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "AL88",
                    description: Some(
                        "Color mode AL88",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "L4",
                    description: Some(
                        "Color mode L4",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "A8",
                    description: Some(
                        "Color mode A8",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "A4",
                    description: Some(
                        "Color mode A4",
                    ),
                    value: 10,
                },
            ],
        },
        Enum {
            name: "FgpfccrStart",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "START",
                    description: Some(
                        "Start the automatic loading of the CLUT",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MEMORY_TO_MEMORY",
                    description: Some(
                        "Memory-to-memory (FG fetch only)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEMORY_TO_MEMORY_PFC",
                    description: Some(
                        "Memory-to-memory with PFC (FG fetch only with FG PFC active)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEMORY_TO_MEMORY_PFCBLENDING",
                    description: Some(
                        "Memory-to-memory with blending (FG and BG fetch with PFC and blending)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "REGISTER_TO_MEMORY",
                    description: Some(
                        "Register-to-memory",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "OpfccrCm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "ARGB4444",
                    ),
                    value: 4,
                },
            ],
        },
    ],
};
                