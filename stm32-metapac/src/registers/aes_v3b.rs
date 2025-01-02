
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Aes",
        extends: None,
        description: Some("Advanced encryption standard hardware accelerator"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "dinr",
                description: Some("Data input register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "doutr",
                description: Some("Data output register"),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "keyr",
                description: Some("Key register"),
                array: Some(Array::Cursed(CursedArray {
                    offsets: &[0, 4, 8, 12, 32, 36, 40, 44],
                })),
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "ivr",
                description: Some("Initialization vector register"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "suspr",
                description: Some("Suspend register"),
                array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                byte_offset: 0x40,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "ier",
                description: Some("interrupt enable register"),
                array: None,
                byte_offset: 0x300,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ier"),
                }),
            },
            BlockItem {
                name: "isr",
                description: Some("interrupt status register"),
                array: None,
                byte_offset: 0x304,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Isr"),
                }),
            },
            BlockItem {
                name: "icr",
                description: Some("interrupt clear register"),
                array: None,
                byte_offset: 0x308,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Icr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("AES enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datatype",
                    description: Some("Data type selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Datatype"),
                },
                Field {
                    name: "mode",
                    description: Some("Operating mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Mode"),
                },
                Field {
                    name: "chmod",
                    description: Some("Chaining mode selection"),
                    bit_offset: BitOffset::Cursed(CursedBitOffset {
                        ranges: &[5..=6, 16..=16],
                    }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Chmod"),
                },
                Field {
                    name: "dmainen",
                    description: Some("Enable DMA management of data input phase"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaouten",
                    description: Some("Enable DMA management of data output phase"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gcmph",
                    description: Some("GCM or CCM phase selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Gcmph"),
                },
                Field {
                    name: "keysize",
                    description: Some("Key size selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "npblb",
                    description: Some("Number of padding bytes in last block of payload"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "kmod",
                    description: Some("Key mode selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iprst",
                    description: Some("AES peripheral software reset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some("Interrupt clear register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rweif",
                    description: Some("Read or write error interrupt flag clear"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "keif",
                    description: Some("Key error interrupt flag clear"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some("Interrupt enable register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccfie",
                    description: Some("Computation complete flag interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rweie",
                    description: Some("Read or write error interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "keie",
                    description: Some("Key error interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some("Interrupt status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccf",
                    description: Some("Computation complete flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rweif",
                    description: Some("Read or write error interrupt flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "keif",
                    description: Some("Key error interrupt flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccf",
                    description: Some("Computation complete flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderr",
                    description: Some("Read error flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrerr",
                    description: Some("Write error flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some("Busy flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "keyvalid",
                    description: Some("Key valid flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Chmod",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ECB",
                    description: Some("Electronic codebook"),
                    value: 0,
                },
                EnumVariant {
                    name: "CBC",
                    description: Some("Cipher-block chaining"),
                    value: 1,
                },
                EnumVariant {
                    name: "CTR",
                    description: Some("Counter mode"),
                    value: 2,
                },
                EnumVariant {
                    name: "GCM_GMAC",
                    description: Some("Galois counter mode and Galois message authentication code"),
                    value: 3,
                },
                EnumVariant {
                    name: "CCM",
                    description: Some("Counter with CBC-MAC"),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Datatype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some("Word"),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF_WORD",
                    description: Some("Half-word (16-bit)"),
                    value: 1,
                },
                EnumVariant {
                    name: "BYTE",
                    description: Some("Byte (8-bit)"),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT",
                    description: Some("Bit"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Gcmph",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INIT_PHASE",
                    description: Some("Init phase"),
                    value: 0,
                },
                EnumVariant {
                    name: "HEADER_PHASE",
                    description: Some("Header phase"),
                    value: 1,
                },
                EnumVariant {
                    name: "PAYLOAD_PHASE",
                    description: Some("Payload phase"),
                    value: 2,
                },
                EnumVariant {
                    name: "FINAL_PHASE",
                    description: Some("Final phase"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MODE1",
                    description: Some("Encryption"),
                    value: 0,
                },
                EnumVariant {
                    name: "MODE2",
                    description: Some("Key derivation (or key preparation for ECB/CBC decryption)"),
                    value: 1,
                },
                EnumVariant {
                    name: "MODE3",
                    description: Some("Decryption"),
                    value: 2,
                },
            ],
        },
    ],
};
