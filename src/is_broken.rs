use core::str;
use std::cmp::Ordering;

use capnp::schema_capnp::field;
use capnp::schema_capnp::type_;
use capnp::schema_capnp::value;
use capnp::schema_capnp::node;
use capnp::schema_capnp::node::annotation;
use capnp::schema_capnp::node::const_;
use capnp::schema_capnp::node::enum_;
use capnp::schema_capnp::node::interface;
use capnp::schema_capnp::node::struct_;
use capnp::raw::*;
use capnpc::codegen::getter_text;
use capnpc::codegen::*;
use capnp::message;
use capnpc::codegen::GeneratorContext;

pub fn is_broken(
    base_ctx: &GeneratorContext,
    changed_ctx: &GeneratorContext,
    node_id: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let base_node = base_ctx.node_map[&node_id];
    let changed_node = changed_ctx.node_map.get(&node_id);
    if changed_node.is_none() {
        println!("{} is broken.", base_node.get_display_name()?);
        return Ok(false);
    }
    let changed_node = changed_node.unwrap();

    let mut ret = false;
    match base_node.which()? {
        node::File(_) => {
            // Files other than the specified are considered to have no differences.
        }
        node::Struct(base_struct) => {
            if let node::Struct(changed_struct) = changed_node.which()? {
                ret = is_broken_struct(base_ctx, base_struct, changed_ctx, changed_struct)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(true);
            }
        }
        node::Interface(base_interface) => {
            if let node::Interface(changed_interface) = changed_node.which()? {
                ret =
                    is_broken_interface(base_ctx, base_interface, changed_ctx, changed_interface)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(true);
            }
        }
        node::Const(base_const) => {
            if let node::Const(changed_const) = changed_node.which()? {
                ret = is_broken_const(base_ctx, base_const, changed_ctx, changed_const)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(true);
            }
        }
        node::Enum(base_enum) => {
            if let node::Enum(changed_enum) = changed_node.which()? {
                ret = is_broken_enum(base_ctx, base_enum, changed_ctx, changed_enum)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(true);
            }
        }
        node::Annotation(_) => {
            // annotation doesn't cause breaking change.
        }
    }
    if ret == true {
        println!("{} is broken", base_node.get_display_name()?);
        return Ok(true);
    }

    for nested_node in base_node.get_nested_nodes().unwrap() {
        ret = is_broken(base_ctx, changed_ctx, nested_node.get_id())?;
    }

    Ok(ret)
}

pub fn is_broken_struct(
    base_ctx: &GeneratorContext,
    base_struct: struct_::Reader,
    changed_ctx: &GeneratorContext,
    changed_struct: struct_::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Please note that  .fields() method returns field iterator ordered by `@` annotation in the schema.
    base_struct
        .get_fields()?
        .iter()
        .zip(changed_struct.get_fields()?.iter())
        .fold(Ok(false), |sum: Result<bool, Box<dyn std::error::Error>>, (x, y)| Ok(sum? || is_broken_field(base_ctx, x, changed_ctx, y)?))
}

pub fn is_broken_interface(
    _: &GeneratorContext,
    _: interface::Reader,
    _: &GeneratorContext,
    _: interface::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    //
    Ok(false)
}

pub fn is_broken_const(
    _: &GeneratorContext,
    _: const_::Reader,
    _: &GeneratorContext,
    _: const_::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    //
    Ok(false)
}

pub fn is_broken_enum(
    _: &GeneratorContext,
    _: enum_::Reader,
    _: &GeneratorContext,
    _: enum_::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    //
    Ok(false)
}

pub fn is_broken_field(
    base_ctx: &GeneratorContext,
    base_field: field::Reader,
    changed_ctx: &GeneratorContext,
    changed_field: field::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    let is_not_broken = 
        base_field.get_name()? == changed_field.get_name()? &&
        base_field.get_discriminant_value() == changed_field.get_discriminant_value();
    if !is_not_broken{
        println!("{} is broken", base_field.get_name()?);
        return Ok(true)
    }
    return match base_field.which()? {
        field::Which::Group(base_group) => {
            if let field::Which::Group(changed_group) = changed_field.which()?{
                is_broken_group(base_ctx, base_group, changed_ctx, changed_group)
            } else {
                println!("{} is broken", base_field.get_name()?);
                Ok(true)
            }
        }
        field::Which::Slot(base_slot) => {
            if let field::Which::Slot(changed_slot) = changed_field.which()?{
                let (a, b, c) = getter_text(&base_ctx, &base_field, true, false)?;
                println!("{}", a);
                is_broken_slot(base_field, base_slot, &changed_field, changed_slot)
            } else {
                println!("{} is broken", base_field.get_name()?);
                Ok(true)
            }
        }
    };
}

pub fn is_broken_group(
    base_ctx: &GeneratorContext,
    base_group: field::group::Reader,
    changed_ctx: &GeneratorContext,
    _: field::group::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    is_broken(base_ctx, changed_ctx, base_group.get_type_id())
}

pub fn is_broken_slot(
    base_field: field::Reader,
    base_slot: field::slot::Reader,
    _: &field::Reader,
    changed_slot: field::slot::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Please note that `@` annotation order is checked at is_broken_struct method.
    if is_broken_type(base_slot.get_type()?, changed_slot.get_type()?)? {
        // type for this slot is changed.
        println!("{} is broken.", base_field.get_name()?);
        return Ok(true);
    }

    // TODO: Default type breaking change detection
    // Even if a new default value is set, it is not a breaking change if it is 0x00000000 in binary representation. 
    // An implementation that takes this into consideration is required.
    if base_slot.has_default_value() != changed_slot.has_default_value() {
        // default value is set or unset.
        println!("{} is broken.", base_field.get_name()?);
        return Ok(true);
    }

    if base_slot.has_default_value() && changed_slot.has_default_value() && is_broken_value(base_slot.get_default_value()?, changed_slot.get_default_value()?)? {
        // default value for this slot is changed.
        println!("{} is broken.", base_field.get_name()?);
        return Ok(true);
    }

    Ok(false)
}

pub fn is_broken_value(
    base_value: value::Reader,
    changed_value: value::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut ret = false;
    match base_value.which()? {
        value::Which::Void(_) => {
            if let value::Which::Void(_) = changed_value.which()? {
            } else {
                ret = true;
            }
        },
        value::Which::Bool(base_bool) => {
            if let value::Which::Bool(changed_bool) = changed_value.which()? {
                ret = base_bool != changed_bool;
            } else {
                ret = true;
            }
        },
        value::Which::Int8(base_i8) => {
            if let value::Which::Int8(changed_i8) = changed_value.which()? {
                ret = base_i8 != changed_i8;
            } else {
                ret = true;
            }
        },
        value::Which::Int16(base_i16) => {
            if let value::Which::Int16(changed_i16) = changed_value.which()? {
                ret = base_i16 != changed_i16;
            } else {
                ret = true;
            }
        },
        value::Which::Int32(base_i32) => {
            if let value::Which::Int32(changed_i32) = changed_value.which()? {
                ret = base_i32 != changed_i32;
            } else {
                ret = true;
            }
        },
        value::Which::Int64(base_i64) => {
            if let value::Which::Int64(changed_i64) = changed_value.which()? {
                ret = base_i64 != changed_i64;
            } else {
                ret = true;
            }
        },
        value::Which::Uint8(base_u8) => {
            if let value::Which::Uint8(changed_u8) = changed_value.which()? {
                ret = base_u8 != changed_u8;
            } else {
                ret = true;
            }
        },
        value::Which::Uint16(base_u16) => {
            if let value::Which::Uint16(changed_u16) = changed_value.which()? {
                ret = base_u16 != changed_u16;
            } else {
                ret = true;
            }
        },
        value::Which::Uint32(base_u32) => {
            if let value::Which::Uint32(changed_u32) = changed_value.which()? {
                ret = base_u32 != changed_u32;
            } else {
                ret = true;
            }
        },
        value::Which::Uint64(base_u64) => {
            if let value::Which::Uint64(changed_u64) = changed_value.which()? {
                ret = base_u64 != changed_u64;
            } else {
                ret = true;
            }
        },
        value::Which::Float32(base_f32) => {
            if let value::Which::Float32(changed_f32) = changed_value.which()? {
                ret = base_f32 != changed_f32;
            } else {
                ret = true;
            }
        },
        value::Which::Float64(base_f64) => {
            if let value::Which::Float64(changed_f64) = changed_value.which()? {
                ret = base_f64 != changed_f64;
            } else {
                ret = true;
            }
        },
        value::Which::Text(base_a0) => {
            if let value::Which::Text(changed_a0) = changed_value.which()? {
                ret = base_a0? != changed_a0?;
            } else {
                ret = true;
            }
        },
        value::Which::Data(base_a1) => {
            if let value::Which::Data(changed_a1) = changed_value.which()? {
                ret =  str::from_utf8(base_a1?) != str::from_utf8(changed_a1?);
            } else {
                ret = true;
            }
        },
        value::Which::List(base_list) => {
            if let value::Which::List(changed_list) = changed_value.which()? {
                // TODO: Implement Eq for List value
                ret = base_list.get_as::<&[u8]>()?.cmp(changed_list.get_as::<&[u8]>()?) == Ordering::Equal;
            } else {
                ret = true;
            }
        },
        value::Which::Enum(base_u16) => {
            if let value::Which::Enum(changed_u16) = changed_value.which()? {
                ret = base_u16 != changed_u16;
            } else {
                ret = true;
            }
        },
        value::Which::Struct(_) => {
            if let value::Which::Struct(_) = changed_value.which()? {
                // TODO: Implement Eq for Struct value
            } else if let value::Which::AnyPointer(_) = changed_value.which()? {
                // In this case, field type becomes AnyPointer(or Generics). See N03.
            } else {
                ret = true;
            }
        },
        value::Which::Interface(()) => {
            if let value::Which::Interface(_) = changed_value.which()? {
                // TODO: Implement Eq for Interface value
            } else {
                ret = true;
            }
        },
        value::Which::AnyPointer(_) => {
            if let value::Which::AnyPointer(_) = changed_value.which()? {
                // TODO: Implement Eq for AnyPointer value
            } else {
                ret = true;
            }
        },
    }
    Ok(ret)
}

pub fn is_broken_type(
    base_type: type_::Reader,
    changed_type: type_::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut ret = false;
    match base_type.which()? {
        type_::Which::Void(()) => {
            if let type_::Which::Void(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Bool(()) => {
            if let type_::Which::Bool(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Int8(()) => {
            if let type_::Which::Int8(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Int16(()) => {
            if let type_::Which::Int16(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Int32(()) => {
            if let type_::Which::Int32(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Int64(()) => {
            if let type_::Which::Int64(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Uint8(()) => {
            if let type_::Which::Uint8(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Uint16(()) => {
            if let type_::Which::Uint16(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Uint32(()) => {
            if let type_::Which::Uint32(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Uint64(()) => {
            if let type_::Which::Uint64(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Float32(()) => {
            if let type_::Which::Float32(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Float64(()) => {
            if let type_::Which::Float64(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Text(()) => {
            if let type_::Which::Text(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::Data(()) => {
            if let type_::Which::Data(()) = changed_type.which()? {
            } else {
                ret = true;
            }
        },
        type_::Which::List(base_a0) => {
            if let type_::Which::List(changed_a0) = changed_type.which()? {
                ret = is_broken_type(base_a0.get_element_type()?, changed_a0.get_element_type()?)?;
            } else {
                ret = true;
            }
        },
        type_::Which::Enum(base_a1) => {
            if let type_::Which::Enum(changed_a1) = changed_type.which()? {
                ret = base_a1.get_type_id() != changed_a1.get_type_id();
            } else {
                ret = true;
            }
        },
        type_::Which::Struct(base_a2) => {
            if let type_::Which::Struct(changed_a2) = changed_type.which()? {
                ret = base_a2.get_type_id() != changed_a2.get_type_id();
            } else if let type_::Which::AnyPointer(changed_a2) = changed_type.which()?{
                match changed_a2.which()? {
                    type_::any_pointer::Which::Unconstrained(_) => {
                        ret = true;
                    },
                    type_::any_pointer::Which::Parameter(_) => {
                        // In this case, field type becomes AnyPointer(or Generics). See N03.
                    },
                    type_::any_pointer::Which::ImplicitMethodParameter(_) => {
                        ret = true;
                    },
                }
            } else {
                ret = true;
            }
        },
        type_::Which::Interface(base_a3) => {
            if let type_::Which::Interface(changed_a3) = changed_type.which()? {
                ret = base_a3.get_type_id() != changed_a3.get_type_id();
            } else {
                ret = true;
            }
        },
        type_::Which::AnyPointer(_) => {
            if let type_::Which::AnyPointer(_) = changed_type.which()? {
                // TODO: check if two AnyPointer types are same. 
            } else {
                ret = true;
            }
        },
    }
    Ok(ret)
}

#[allow(dead_code)]
pub fn is_broken_annotation(
    _: &GeneratorContext,
    _: annotation::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    // nop
    Ok(false)
}
