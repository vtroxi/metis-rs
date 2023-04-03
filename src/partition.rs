use crate::{
    idx_t, moptions_et_METIS_OPTION_UFACTOR, Graph, METIS_PartGraphKway, METIS_SetDefaultOptions,
    METIS_NOPTIONS,
};
use std::ptr::null_mut;

pub enum PartitioningMethod {
    MultilevelKWay,
    MultilevelRecursiveBisection,
}

pub struct PartitioningConfig {
    pub method: PartitioningMethod,
    // TODO support all useful metis options here
}

impl Graph {
    pub fn partition(&mut self, config: PartitioningConfig, partitions: u32) {
        let mut n = self.vertices.len() as idx_t;
        let mut adjacency = Vec::new(); // adjncy
        let mut adjacency_weight = Vec::new(); // adjcwgt
        let mut adjacency_idx = Vec::new(); // xadj
        for v in self.vertices.iter() {
            adjacency_idx.push(adjacency.len() as idx_t);
            for e in v.edges.iter() {
                adjacency.push(e.dst as idx_t);
                adjacency_weight.push(e.weight as idx_t)
            }
        }
        adjacency_idx.push(adjacency.len() as idx_t);

        let mut part = vec![0 as idx_t; self.vertices.len()];
        let mut edge_cut = 0 as idx_t;
        let mut nparts = partitions as idx_t;
        let mut num_constraints = 1 as idx_t;

        let mut options = [0 as idx_t; METIS_NOPTIONS as usize];
        unsafe {
            METIS_SetDefaultOptions(&mut options as *mut idx_t);
        }
        options[moptions_et_METIS_OPTION_UFACTOR as usize] = 200;

        unsafe {
            METIS_PartGraphKway(
                &mut n,
                &mut num_constraints,
                adjacency_idx.as_mut_ptr(),
                adjacency.as_mut_ptr(),
                null_mut(),
                null_mut(),
                adjacency_weight.as_mut_ptr(),
                &mut nparts,
                null_mut(),
                null_mut(),
                options.as_mut_ptr(),
                &mut edge_cut,
                part.as_mut_ptr(),
            );
        }

        for (i, &p) in part.iter().enumerate() {
            self.vertices[i].color = p as u32;
        }
    }
}
