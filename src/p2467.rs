use std::{collections::{HashMap, HashSet}, fs};

pub fn run() {
    for i in [
        // (vec![vec![0,1], vec![1,2], vec![1,3], vec![3,4]], 3, vec![-2,4,2,-4,6]),
        // (vec![vec![0,1]], 1, vec![-7280,2350]),
        // (vec![vec![0,2],vec![0,4],vec![1,3],vec![1,2]], 1, vec![3958,-9854,-8334,-9388,3410])
        (
            serde_json::from_str::<Vec<Vec<i32>>>(&fs::read_to_string("./src/too_damn_long.txt").unwrap().split("\n").map(|x| x.to_string()).collect::<Vec<String>>()[0]).unwrap(),
            serde_json::from_str::<i32>(&fs::read_to_string("./src/too_damn_long.txt").unwrap().split("\n").map(|x| x.to_string()).collect::<Vec<String>>()[1]).unwrap(),
            serde_json::from_str::<Vec<i32>>(&fs::read_to_string("./src/too_damn_long.txt").unwrap().split("\n").map(|x| x.to_string()).collect::<Vec<String>>()[2]).unwrap(),
        )
    ] {
        println!("{}", most_profitable_path(i.0, i.1, i.2));
    }
}

pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
    let mut amount = amount;

    let mut neighbours: Vec<Vec<usize>> = vec![vec![]; amount.len()];
    for i in edges {
        neighbours[i[0] as usize].push(i[1] as usize);
        neighbours[i[1] as usize].push(i[0] as usize);
    }

    let mut distance_map: Vec<usize> = vec![0; amount.len()];
    let mut parent_map: Vec<usize> = vec![0; amount.len()];
    determine_dist_parent(0, 0, 0, &mut distance_map, &mut parent_map, &neighbours);

    get_bob_path(bob as usize, &parent_map, &mut amount, &distance_map);

    println!("{distance_map:?}");
    println!("{amount:?}");

    0
}

fn determine_dist_parent(node: usize, parent: usize, depth: usize, distance_map: &mut Vec<usize>, parent_map: &mut Vec<usize>, neighbours: &Vec<Vec<usize>>) {
    distance_map[node] = depth;
    parent_map[node] = parent;

    for n in &neighbours[node] {
        if *n == parent {
            continue;
        }
        determine_dist_parent(*n, node, depth + 1, distance_map, parent_map, neighbours);
    }
}

fn get_bob_path(start: usize, parents: &Vec<usize>, amount: &mut Vec<i32>, distance_map: &Vec<usize>) {
    // assumption : node 0 is a root node, and all nodes come from it
    // from this assumption, we can just go through all parents from bob's node to reach 0
    let mut position = start;
    let mut depth = 0;

    while position != 0 {
        // mutate amount vector as well; optimisation taken from the other solutions
        let d = distance_map[position];
        if d == depth {
            amount[position] = amount[position] / 2;
        } else if d > depth {
            amount[position] = 0;
        }

        depth += 1;
        position = parents[position];
        // dont need to worry about mutating amount vector when position == 0
        // the only case it matters is when there is only one node, 0, and that wont happen
    }
}

fn dfs(node: usize, childrens: &HashMap<usize, Vec<usize>>, amount_map: &Vec<i32>, bob_traversed: &HashMap<usize, usize>, current_amount: i32, result: &mut i32, depth: usize, parent: usize) {
    let mut current_amount = current_amount;

    match bob_traversed.get(&node) {
        Some(b) => {
            if depth == *b {
                current_amount += (amount_map[node] as f32 * 0.5) as i32;
            } else {
                if depth < *b {
                    current_amount += amount_map[node];
                }
            }
        },
        None => {
            current_amount += amount_map[node];
        }
    }

    println!("{current_amount} at {node}");

    match childrens.get(&node) {
        Some(children) => {
            let mut flag = false;
            for c in children {
                if *c == parent {
                    continue;
                }

                flag = true;
                dfs(*c, childrens, amount_map, bob_traversed, current_amount, result, depth + 1, node);
            }

            if !flag {
                if current_amount > *result {
                    *result = current_amount;
                }
            }
        },
        None => {
            if current_amount > *result {
                *result = current_amount;
            }
        }
    }
}

// pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
//     let mut children: HashMap<usize, Vec<usize>> = HashMap::new();

//     for i in edges {
//         match children.get_mut(&(i[0] as usize)) {
//             Some(c) => {
//                 c.push(i[1] as usize);
//             },
//             None => {
//                 children.insert(i[0] as usize, vec![i[1] as usize]);
//             }
//         }
//         match children.get_mut(&(i[1] as usize)) {
//             Some(c) => {
//                 c.push(i[0] as usize);
//             },
//             None => {
//                 children.insert(i[1] as usize, vec![i[0] as usize]);
//             }
//         }
//     }

//     // assumption : node 0 is a root node, and all nodes come from it
//     // from this assumption, we can just go through all parents from bob's node to reach 0
//     // 
//     // assumption incorrect
//     // the graph is an indirect graph, so there is no direct parent-child relation
//     // the child can be the parent, and vice versa
//     // bfs and find the first path to reach node 0
//     let bob_traversed = get_bob_path(bob as usize, &children);

//     // println!("{bob_traversed:?}");

//     let mut result: i32 = std::i32::MIN; // highest net income
//     dfs(0, &children, &amount, &bob_traversed, 0, &mut result, 0, 0);

//     result
// }

// fn get_bob_path(start: usize, childrens: &HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
//     let mut traversed = HashSet::new();
//     traversed.insert(start);
//     let mut nodes: Vec<(usize, HashMap<usize, usize>)> = vec![(start, HashMap::from_iter(vec![(start, 0)]))];

//     loop {
//         let mut next_nodes = vec![];

//         for n in nodes {
//             match childrens.get(&n.0) {
//                 Some(children) => {
//                     for c in children {
//                         if n.1.contains_key(&c) {
//                             continue;
//                         }

//                         let mut m = n.1.clone();
//                         m.insert(c.clone(), m.len());

//                         if *c == 0 {
//                             return m;
//                         }

//                         next_nodes.push((*c, m));
//                     }
//                 },
//                 None => {
//                     continue;
//                 }
//             }
//         }

//         if next_nodes.is_empty() {
//             break;
//         }

//         nodes = next_nodes;
//     }

//     unreachable!()

//     // let mut bob_traversed: HashMap<usize, usize> = HashMap::new();
//     // bob_traversed.insert(bob as usize, 0);
//     // let mut bob_position = bob as usize;
//     // while bob_position != 0 {
//     //     bob_position = *parent.get(&bob_position).unwrap();
//     //     bob_traversed.insert(bob_position, bob_traversed.len());
//     // }
// }

// fn dfs(node: usize, childrens: &HashMap<usize, Vec<usize>>, amount_map: &Vec<i32>, bob_traversed: &HashMap<usize, usize>, current_amount: i32, result: &mut i32, depth: usize, parent: usize) {
//     let mut current_amount = current_amount;

//     match bob_traversed.get(&node) {
//         Some(b) => {
//             if depth == *b {
//                 current_amount += (amount_map[node] as f32 * 0.5) as i32;
//             } else {
//                 if depth < *b {
//                     current_amount += amount_map[node];
//                 }
//             }
//         },
//         None => {
//             current_amount += amount_map[node];
//         }
//     }

//     println!("{current_amount} at {node}");

//     match childrens.get(&node) {
//         Some(children) => {
//             let mut flag = false;
//             for c in children {
//                 if *c == parent {
//                     continue;
//                 }

//                 flag = true;
//                 dfs(*c, childrens, amount_map, bob_traversed, current_amount, result, depth + 1, node);
//             }

//             if !flag {
//                 if current_amount > *result {
//                     *result = current_amount;
//                 }
//             }
//         },
//         None => {
//             if current_amount > *result {
//                 *result = current_amount;
//             }
//         }
//     }
// }


// vector<vector<int>>adj;
// vector<int>par,dis;
// //Find the parent and distance from node 0
// void dfs(int u,int p = 0,int d = 0){
//     dis[u] = d;
//     par[u] = p;
//     for(int v:adj[u]){
//         if(v==p)continue;
//         dfs(v,u,d+1);
//     }
// }
// // Find total sum to each node
// int dfs2(int u,vector<int>&amount,int p= 0){
//     int ret = amount[u];
//     int mxc = -INT_MAX;
//     for(int v:adj[u]){
//         if(v!=p){
//             mxc= max(mxc,dfs2(v,amount,u));
//         }
//     }
//     //if the node is leaf we just return its amount
//     if(mxc==-INT_MAX)return ret;
//     else return ret+mxc;
// }
// int mostProfitablePath(vector<vector<int>>& edges, int bob, vector<int>& amount) {
//     int n = amount.size();
//     adj.resize(n,vector<int>());
//     for(auto&e:edges){
//         adj[e[0]].push_back(e[1]);
//         adj[e[1]].push_back(e[0]);
//     }
//     par.resize(n);
//     dis.resize(n);
//     dfs(0);
//     int cur = bob;
//     int bob_dis = 0;
//     //update the path of from Bob to 0
//     while(cur!=0){
//         if(dis[cur]>bob_dis){
//             amount[cur] = 0;
//         }else if(dis[cur]==bob_dis){
//             amount[cur]/=2;
//         }
//         cur = par[cur];
//         bob_dis++;
//     }
//     return dfs2(0,amount);
// }