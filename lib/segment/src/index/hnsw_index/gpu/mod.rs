pub mod gpu_candidates_heap;
pub mod gpu_graph_builder;
pub mod gpu_links;
pub mod gpu_nearest_heap;
pub mod gpu_search_context;
pub mod gpu_vector_storage;
pub mod gpu_visited_flags;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static GPU_INDEXING: AtomicBool = AtomicBool::new(false);
static GPU_MAX_GROUPS_COUNT: AtomicUsize = AtomicUsize::new(GPU_MAX_GROUPS_COUNT_DEFAULT);
pub const GPU_MAX_GROUPS_COUNT_DEFAULT: usize = 256;

pub fn set_gpu_indexing(gpu_indexing: bool) {
    GPU_INDEXING.store(gpu_indexing, Ordering::Relaxed);
}

pub fn get_gpu_indexing() -> bool {
    GPU_INDEXING.load(Ordering::Relaxed)
}

pub fn set_gpu_max_groups_count(count: usize) {
    GPU_MAX_GROUPS_COUNT.store(count, Ordering::Relaxed);
}

pub fn get_gpu_max_groups_count() -> usize {
    GPU_MAX_GROUPS_COUNT.load(Ordering::Relaxed)
}
