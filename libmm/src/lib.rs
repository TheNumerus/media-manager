pub mod api;
pub mod db;
pub mod error;
pub mod media;

pub trait EntityState {}

pub struct Incomplete;
impl EntityState for Incomplete {}

pub struct Complete;
impl EntityState for Complete {}

pub struct Loaded;
impl EntityState for Loaded {}
