
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Exti",
        extends: None,
        description: Some("External interrupt/event controller"),
        items: &[
            BlockItem {
                name: "emr",
                description: Some("Interrupt mask register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "ftsr",
                description: Some("Falling Trigger selection register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "imr",
                description: Some("Interrupt mask register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "pr",
                description: Some("Pending register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "rtsr",
                description: Some("Rising Trigger selection register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
            BlockItem {
                name: "swier",
                description: Some("Software interrupt event register"),
                array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lines"),
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "Lines",
        extends: None,
        description: Some("EXTI lines register, 1 bit per line"),
        bit_size: 32,
        fields: &[Field {
            name: "line",
            description: Some("EXTI line"),
            bit_offset: 0,
            bit_size: 1,
            array: Some(Array::Regular(RegularArray { len: 32, stride: 1 })),
            enumm: None,
        }],
    }],
    enums: &[],
};
