pub mod collection;
pub mod collection_manager;
pub mod collection_state;
pub mod common;
pub mod config;
pub mod discovery;
pub mod grouping;
pub mod hash_ring;
pub mod lookup;
pub mod operations;
pub mod optimizers_builder;
pub mod recommendations;
pub mod save_on_disk;
pub mod shards;
pub mod telemetry;
mod update_handler;
pub mod wal;
pub mod wal_delta;

pub mod events;
#[cfg(test)]
mod tests;

// TODO: remove this!
pub type BitSliceElement = ::common::bitvec::BitSliceElement;
pub type BitSlice = ::common::bitvec::BitSlice;
pub type BitVec = ::common::bitvec::BitVec;
