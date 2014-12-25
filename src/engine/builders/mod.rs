use engine::Map;

pub use self::string_builder::MapFromString;

mod string_builder;

pub trait MapBuilder {
    fn build(&self) -> Map;
}
