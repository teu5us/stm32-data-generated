
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "Flash",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "Access control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keyr",
                    description: Some(
                        "Flash key register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "Option byte key register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Flash control register",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                    name: "eccr",
                    description: Some(
                        "Flash ECC register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optr",
                    description: Some(
                        "Flash option register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1ar",
                    description: Some(
                        "Flash WRP area A address register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp1ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1br",
                    description: Some(
                        "Flash WRP area B address register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp1br",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp2ar",
                    description: Some(
                        "Flash WRP area A address register bank 2",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp2ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp2br",
                    description: Some(
                        "Flash WRP area B address register bank 2",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp2br",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some(
                "Access control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "Latency",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Latency",
                    ),
                },
                Field {
                    name: "prften",
                    description: Some(
                        "Prefetch enable",
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
                    name: "icen",
                    description: Some(
                        "Instruction cache enable",
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
                    name: "icrst",
                    description: Some(
                        "Instruction cache reset",
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
                    name: "empty",
                    description: Some(
                        "Flash User area empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Flash control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Programming",
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
                    name: "per",
                    description: Some(
                        "Page erase",
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
                    name: "mer",
                    description: Some(
                        "Mass erase",
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
                    name: "pnb",
                    description: Some(
                        "Page number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bker",
                    description: Some(
                        "Bank number",
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
                    name: "mer2",
                    description: Some(
                        "Mass erase bank 2",
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
                    name: "strt",
                    description: Some(
                        "Start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optstrt",
                    description: Some(
                        "Options modification start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fstpg",
                    description: Some(
                        "Fast programming",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some(
                        "End of operation interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errie",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "obl_launch",
                    description: Some(
                        "Force the option byte loading",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optlock",
                    description: Some(
                        "Options Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "FLASH_CR Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccr",
            extends: None,
            description: Some(
                "Flash ECC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC fail address",
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
                Field {
                    name: "sysf_ecc",
                    description: Some(
                        "ECC fail for Corrected ECC Error or Double ECC Error in info block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccie",
                    description: Some(
                        "ECC correction interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccc",
                    description: Some(
                        "ECC correction",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccd",
                    description: Some(
                        "ECC detection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Optr",
            extends: None,
            description: Some(
                "Flash option register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdp",
                    description: Some(
                        "Read protection level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Rdp",
                    ),
                },
                Field {
                    name: "n_rst_stop",
                    description: Some(
                        "nRST_STOP",
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
                    name: "n_rst_stdby",
                    description: Some(
                        "nRST_STDBY",
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
                    name: "idwg_sw",
                    description: Some(
                        "Independent watchdog selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stop",
                    description: Some(
                        "Independent watchdog counter freeze in Stop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stdby",
                    description: Some(
                        "Independent watchdog counter freeze in Standby mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdg_sw",
                    description: Some(
                        "Window watchdog selection",
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
                Field {
                    name: "dual_bank",
                    description: Some(
                        "Dual bank operation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ram_parity_check",
                    description: Some(
                        "SRAM parity check control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot_sel",
                    description: Some(
                        "nBOOT_SEL",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot1",
                    description: Some(
                        "Boot configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot0",
                    description: Some(
                        "nBOOT0 option bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some(
                        "End of operation",
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
                    name: "operr",
                    description: Some(
                        "Operation error",
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
                    name: "progerr",
                    description: Some(
                        "Programming error",
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
                    name: "wrperr",
                    description: Some(
                        "Write protected error",
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
                    name: "pgaerr",
                    description: Some(
                        "Programming alignment error",
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
                Field {
                    name: "sizerr",
                    description: Some(
                        "Size error",
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
                    name: "pgserr",
                    description: Some(
                        "Programming sequence error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "miserr",
                    description: Some(
                        "Fast programming data miss error",
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
                    name: "fasterr",
                    description: Some(
                        "Fast programming error",
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
                    name: "optverr",
                    description: Some(
                        "Option and Engineering bits loading validity error",
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
                    name: "bsy",
                    description: Some(
                        "Busy",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bsy2",
                    description: Some(
                        "Busy bank 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfgbsy",
                    description: Some(
                        "Programming or erase configuration busy.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp1ar",
            extends: None,
            description: Some(
                "Flash WRP area A address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1a_strt",
                    description: Some(
                        "WRP area A start offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1a_end",
                    description: Some(
                        "WRP area A end offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp1br",
            extends: None,
            description: Some(
                "Flash WRP area B address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1b_strt",
                    description: Some(
                        "WRP area B start offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1b_end",
                    description: Some(
                        "WRP area B end offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp2ar",
            extends: None,
            description: Some(
                "Flash WRP area A address register bank 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp2a_strt",
                    description: Some(
                        "WRP area A start offset bank 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp2a_end",
                    description: Some(
                        "WRP area A end offset bank 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp2br",
            extends: None,
            description: Some(
                "Flash WRP area B address register bank 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp2b_strt",
                    description: Some(
                        "WRP area B start offset bank 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp2b_end",
                    description: Some(
                        "WRP area B end offset bank 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Latency",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "WS0",
                    description: Some(
                        "Zero wait states",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WS1",
                    description: Some(
                        "One wait state",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WS2",
                    description: Some(
                        "Two wait states",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Rdp",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "LEVEL_0",
                    description: Some(
                        "Read protection not active",
                    ),
                    value: 170,
                },
                EnumVariant {
                    name: "LEVEL_1",
                    description: Some(
                        "Memories read protection active",
                    ),
                    value: 187,
                },
                EnumVariant {
                    name: "LEVEL_2",
                    description: Some(
                        "Chip read protection active",
                    ),
                    value: 204,
                },
            ],
        },
    ],
};
                