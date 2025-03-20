use std::collections::HashMap;

pub fn run() {
    for i in [
        // (5, vec![vec![0,1,7],vec![1,3,7],vec![1,2,1]], vec![vec![0,3],vec![3,4]]),
        // (3, vec![vec![0,2,7],vec![0,1,15],vec![1,2,6],vec![1,2,1]], vec![vec![1,2]]),
        (6, vec![vec![1,5,1],vec![4,3,3],vec![3,5,3],vec![1,0,1],vec![3,0,0]], vec![vec![0,2],vec![4,5],vec![5,1],vec![0,4],vec![0,1],vec![0,4],vec![4,2],vec![4,0]])
        // [-1,0,0,0,0,0,-1,0]
    ] {
        println!("{:?}", minimum_cost(i.0, i.1, i.2));
    }
}

pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let mut representatives: Vec<usize> = (0..(n as usize)).collect();
    let mut weightage: HashMap<usize, i32> = HashMap::new();

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

    for i in &edges {
        union(i[0] as usize, i[1] as usize, &mut representatives, &mut level);
    }

    for i in &edges {
        let p = get_representative(i[0] as usize, &mut representatives);
        match weightage.get_mut(&p) {
            Some(w) => {
                *w &= i[2];
            },
            None => {
                weightage.insert(p, i[2]);
            }
        }
    }

    let mut result = vec![];

    for i in query {
        let r_a = get_representative(i[0] as usize, &mut representatives);
        let r_b = get_representative(i[1] as usize, &mut representatives);

        if r_a != r_b {
            result.push(-1);
        } else {
            result.push(*weightage.get(&r_a).unwrap());
        }
    }

    result
}