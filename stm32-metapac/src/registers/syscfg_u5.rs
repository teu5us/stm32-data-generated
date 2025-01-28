
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller",
            ),
            items: &[
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "SYSCFG secure configuration\r register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpuimr",
                    description: Some(
                        "FPU interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpuimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnslckr",
                    description: Some(
                        "SYSCFG CPU non-secure lock\r register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnslckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cslockr",
                    description: Some(
                        "SYSCFG CPU secure lock\r register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cslockr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mesr",
                    description: Some(
                        "memory erase status register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mesr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccsr",
                    description: Some(
                        "compensation cell control/status register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccvr",
                    description: Some(
                        "compensation cell value register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccr",
                    description: Some(
                        "compensation cell code register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsscmdr",
                    description: Some(
                        "RSS command register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsscmdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ucpdr",
                    description: Some(
                        "USB Type C and Power Delivery register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ucpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otghsphycr",
                    description: Some(
                        "OTG_HS PHY register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otghsphycr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otghsphytuner2",
                    description: Some(
                        "OTG_HS PHY tune register 2",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otghsphytuner2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cccr",
            extends: None,
            description: Some(
                "compensation cell code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncc1",
                    description: Some(
                        "NCC1",
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
                    name: "pcc1",
                    description: Some(
                        "PCC1",
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
                    name: "ncc2",
                    description: Some(
                        "NCC2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcc2",
                    description: Some(
                        "PCC2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cccsr",
            extends: None,
            description: Some(
                "compensation cell control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "EN1",
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
                    name: "cs1",
                    description: Some(
                        "CS1",
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
                    name: "en2",
                    description: Some(
                        "EN2",
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
                    name: "cs2",
                    description: Some(
                        "CS2",
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
                    name: "rdy1",
                    description: Some(
                        "RDY1",
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
                    name: "rdy2",
                    description: Some(
                        "RDY2",
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
            ],
        },
        FieldSet {
            name: "Ccvr",
            extends: None,
            description: Some(
                "compensation cell value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncv1",
                    description: Some(
                        "NCV1",
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
                    name: "pcv1",
                    description: Some(
                        "PCV1",
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
                    name: "ncv2",
                    description: Some(
                        "NCV2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcv2",
                    description: Some(
                        "PCV2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boosten",
                    description: Some(
                        "I/O analog switch voltage booster\r enable",
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
                    name: "anaswvdd",
                    description: Some(
                        "GPIO analog switch control voltage\r selection",
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
                    name: "pb6_fmp",
                    description: Some(
                        "PB6_FMP",
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
                    name: "pb7_fmp",
                    description: Some(
                        "PB7_FMP",
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
                    name: "pb8_fmp",
                    description: Some(
                        "PB8_FMP",
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
                    name: "pb9_fmp",
                    description: Some(
                        "PB9_FMP",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cll",
                    description: Some(
                        "LOCKUP (hardfault) output enable\r bit",
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
                    name: "spl",
                    description: Some(
                        "SRAM ECC lock bit",
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
                    name: "pvdl",
                    description: Some(
                        "PVD lock enable bit",
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
                    name: "eccl",
                    description: Some(
                        "ECC Lock",
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
            ],
        },
        FieldSet {
            name: "Cnslckr",
            extends: None,
            description: Some(
                "SYSCFG CPU non-secure lock\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locknsvtor",
                    description: Some(
                        "VTOR_NS register lock",
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
                    name: "locknsmpu",
                    description: Some(
                        "Non-secure MPU registers\r lock",
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
            ],
        },
        FieldSet {
            name: "Cslockr",
            extends: None,
            description: Some(
                "SYSCFG CPU secure lock\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locksvtaircr",
                    description: Some(
                        "LOCKSVTAIRCR",
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
                    name: "locksmpu",
                    description: Some(
                        "LOCKSMPU",
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
                    name: "locksau",
                    description: Some(
                        "LOCKSAU",
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
            ],
        },
        FieldSet {
            name: "Fpuimr",
            extends: None,
            description: Some(
                "FPU interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpu_ie",
                    description: Some(
                        "Floating point unit interrupts enable\r bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mesr",
            extends: None,
            description: Some(
                "memory erase status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mclr",
                    description: Some(
                        "MCLR",
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
                    name: "ipmee",
                    description: Some(
                        "IPMEE",
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
            name: "Otghsphycr",
            extends: None,
            description: Some(
                "OTG_HS PHY register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "PHY Enable",
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
                    name: "pdctrl",
                    description: Some(
                        "Common block power-down control",
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
                    name: "clksel",
                    description: Some(
                        "Reference clock frequency selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Usbrefcksel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Otghsphytuner2",
            extends: None,
            description: Some(
                "OTG_HS tune register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "compdistune",
                    description: Some(
                        "Disconnect threshold adjustment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sqrxtune",
                    description: Some(
                        "Squelch threshold adjustment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txpreempamptune",
                    description: Some(
                        "HS transmitter preemphasis current control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsscmdr",
            extends: None,
            description: Some(
                "RSS command register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsscmd",
                    description: Some(
                        "RSS commands",
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
            name: "Seccfgr",
            extends: None,
            description: Some(
                "SYSCFG secure configuration\r register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsec",
                    description: Some(
                        "SYSCFG clock control\r security",
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
                    name: "classbsec",
                    description: Some(
                        "CLASSBSEC",
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
                    name: "fpusec",
                    description: Some(
                        "FPUSEC",
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
            ],
        },
        FieldSet {
            name: "Ucpdr",
            extends: None,
            description: Some(
                "USB Type C and Power Delivery register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cc1enrxfilter",
                    description: Some(
                        "CC1ENRXFILTER",
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
                    name: "cc2enrxfilter",
                    description: Some(
                        "CC2ENRXFILTER",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Usbrefcksel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "MHZ16",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 16 MHz.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MHZ19_2",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 19.2 MHz.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MHZ20",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 20MHz.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MHZ24",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 24 MHz (default after reset).",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "MHZ32",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 32 MHz.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "MHZ26",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 26 MHz.",
                    ),
                    value: 14,
                },
            ],
        },
    ],
};
                