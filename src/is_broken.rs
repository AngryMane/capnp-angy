use capnp::schema_capnp::field;
use capnp::schema_capnp::node;
use capnp::schema_capnp::node::annotation;
use capnp::schema_capnp::node::const_;
use capnp::schema_capnp::node::enum_;
use capnp::schema_capnp::node::interface;
use capnp::schema_capnp::node::struct_;
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

    let mut ret = true;
    match base_node.which()? {
        node::File(_) => {
            // Files other than the specified are considered to have no differences.
        }
        node::Struct(base_struct) => {
            if let node::Struct(changed_struct) = changed_node.which()? {
                ret = is_broken_struct(base_ctx, base_struct, changed_ctx, changed_struct)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(false);
            }
        }
        node::Interface(base_interface) => {
            if let node::Interface(changed_interface) = changed_node.which()? {
                ret =
                    is_broken_interface(base_ctx, base_interface, changed_ctx, changed_interface)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(false);
            }
        }
        node::Const(base_const) => {
            if let node::Const(changed_const) = changed_node.which()? {
                ret = is_broken_const(base_ctx, base_const, changed_ctx, changed_const)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(false);
            }
        }
        node::Enum(base_enum) => {
            if let node::Enum(changed_enum) = changed_node.which()? {
                ret = is_broken_enum(base_ctx, base_enum, changed_ctx, changed_enum)?;
            } else {
                println!("{} is broken.", base_node.get_display_name()?);
                return Ok(false);
            }
        }
        node::Annotation(_) => {
            // annotation doesn't cause breaking change.
        }
    }
    if ret == false {
        return Ok(false);
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
    base_struct
        .get_fields()?
        .iter()
        .zip(changed_struct.get_fields()?.iter())
        .fold(Ok(true), |sum: Result<bool, Box<dyn std::error::Error>>, (x, y)| Ok(sum? && is_broken_field(base_ctx, x, changed_ctx, y)?))
}

pub fn is_broken_interface(
    base_ctx: &GeneratorContext,
    base_interface: interface::Reader,
    changed_ctx: &GeneratorContext,
    changed_interface: interface::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    //
    Ok(true)
}

pub fn is_broken_const(
    base_ctx: &GeneratorContext,
    base_const: const_::Reader,
    changed_ctx: &GeneratorContext,
    changed_const: const_::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    //
    Ok(true)
}

pub fn is_broken_enum(
    base_ctx: &GeneratorContext,
    base_enum: enum_::Reader,
    changed_ctx: &GeneratorContext,
    changed_enum: enum_::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    //
    Ok(true)
}

pub fn is_broken_field(
    _: &GeneratorContext,
    base_field: field::Reader,
    _: &GeneratorContext,
    changed_field: field::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    let ret = base_field.get_code_order() == changed_field.get_code_order() &&
        base_field.get_name()? == changed_field.get_name()? &&
        base_field.get_discriminant_value() == changed_field.get_discriminant_value();
    Ok(ret)
}

#[allow(dead_code)]
pub fn is_broken_annotation(
    _: &GeneratorContext,
    _: annotation::Reader,
) -> Result<bool, Box<dyn std::error::Error>> {
    // nop
    Ok(true)
}
