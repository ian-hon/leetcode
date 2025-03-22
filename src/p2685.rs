use std::collections::HashMap;

pub fn run() {
    for i in [
        (6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4]]),
        (6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4],vec![3,5]])
    ] {
        println!("{}", count_complete_components(i.0, i.1));
    }
}

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut representatives: Vec<usize> = (0..(n as usize)).collect();

    // to implement rank-optimised uf
    let mut level: Vec<usize> = vec![1; n as usize];

    pub fn get_representative(i: usize, r: &mut Vec<usize>) -> usize {
        let p = r[i as usize];
        if i == p {
            return i;
        }
        r[i] = get_representative(p, r);
        r[i]
    }

    pub fn union(a: usize, b: usize, r: &mut Vec<usize>, level: &mut Vec<usize>) {
        let r_a = get_representative(a, r);
        let r_b = get_representative(b, r);

        if r_a == r_b {
            // both have same representatives, both are in the same group
            return;
        }

        if level[r_a] < level[r_b] {
            r[r_a] = r_b;
            level[r_b] += level[r_a];
        } else {
            r[r_b] = r_a;
            level[r_a] += level[r_b];
        }
    }

    let mut edge_count= vec![0; n as usize];

    for i in &edges {
        union(i[0] as usize, i[1] as usize, &mut representatives, &mut level);
        edge_count[i[0] as usize] += 1;
    }

    let mut grouping_count: HashMap<usize, (i32, i32)> = HashMap::new();
    // number of vertices, number of edges

    for i in 0..n {
        let r =  get_representative(i as usize, &mut representatives);

        match grouping_count.get_mut(&r) {
            Some(g) => {
                g.0 += 1;
                g.1 += edge_count[i as usize];
            },
            None => {
                grouping_count.insert(r, (1, edge_count[i as usize]));
            }
        }
    }

    let mut result = 0;

    for (_, v) in grouping_count {
        if (v.0 == 1) && (v.1 == 0) {
            result += 1;
            continue;
        }

        let e = v.0 - 1;

        if v.1 == (((e * e) + e) / 2) {
            result += 1;
        }
    }

    result
}