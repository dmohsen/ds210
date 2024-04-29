use rand::{self, Rng, seq::SliceRandom};

pub fn simulate_random_walks(graph: &[Vec<usize>], vertex_count: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut visit_counts = vec![0; vertex_count];

    for start in 0..vertex_count {
        for _ in 0..100 {
            let mut current = start;
            for _ in 0..100 {
                if !graph[current].is_empty() {
                    if rng.gen_ratio(9, 10) {
                        current = *graph[current].choose(&mut rng).unwrap();
                    } else {
                        current = rng.gen_range(0..vertex_count);
                    }
                } else {
                    current = rng.gen_range(0..vertex_count);
                }
            }
            visit_counts[current] += 1;
        }
    }

    visit_counts
}

pub fn compute_pagerank(visit_counts: &[usize], total_walks: usize) -> Vec<(usize, f64)> {
    let mut rankings = visit_counts.iter().enumerate()
        .map(|(index, &count)| (index, count as f64 / total_walks as f64))
        .collect::<Vec<(usize, f64)>>();
    rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    rankings
}
