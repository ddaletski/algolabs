use rand::Rng;
use std::time::Instant;

use union_find::FastUnionUF;

fn main() {
    let n_nodes = 1e5 as usize;
    let max_edges = n_nodes * 2;

    let mut uf = FastUnionUF::new(n_nodes);

    let mut rng = rand::thread_rng();

    let start = Instant::now();
    for _ in 0..max_edges {
        let id1 = rng.gen_range(0..n_nodes);
        let id2 = rng.gen_range(0..n_nodes);

        uf.join(id1, id2);
    }

    println!("joining {} with {} edges took {} ms.", n_nodes, max_edges, start.elapsed().as_millis());
    println!("clusters: {}", uf.clusters().len());
}
