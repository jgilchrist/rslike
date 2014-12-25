use engine::Map;

pub use self::string_builder::MapFromString;

mod string_builder;

pub trait MapBuilder {
    fn build(&self) -> Result<Map, MapBuildError>;
}

pub struct MapBuildError {
    pub desc: &'static str,
    pub detail: Option<String>,
}

pub type MapBuildResult = Result<Map, MapBuildError>;
