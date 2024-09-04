use capnp::schema_capnp::field;
use capnp::any_pointer;
use capnp::message;
use capnpc::codegen::GeneratorContext;


fn get_default_value_as_bytes(
    base_ctx: &GeneratorContext,
    base_field: field::Reader,
    value: any_pointer::Reader,
) {
    let allocator =
        message::HeapAllocator::new().first_segment_words(value.target_size().unwrap().word_count as u32 + 1);
    let mut message = message::Builder::new(allocator);
    let words = message.get_segments_for_output()[0];
    for index in 0..(words.len() / 8) {
        let bytes = &words[(index * 8)..(index + 1) * 8];
    }
}