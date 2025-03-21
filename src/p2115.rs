use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        // (vec!["bread"], vec![vec!["yeast","flour"]], vec!["yeast","flour","corn"]),
        // (vec!["bread","sandwich"], vec![vec!["yeast","flour"],vec!["bread","meat"]], vec!["yeast","flour","meat"]),
        // (vec!["bread","sandwich","burger"], vec![vec!["yeast","flour"],vec!["bread","meat"],vec!["sandwich","meat","bread"]], vec!["yeast","flour","meat"]),
        (vec!["ju","fzjnm","x","e","zpmcz","h","q"], vec![vec!["d"],vec!["hveml","f","cpivl"],vec!["cpivl","zpmcz","h","e","fzjnm","ju"],vec!["cpivl","hveml","zpmcz","ju","h"],vec!["h","fzjnm","e","q","x"],vec!["d","hveml","cpivl","q","zpmcz","ju","e","x"],vec!["f","hveml","cpivl"]], vec!["f","hveml","cpivl","d"]),
        (vec!["g","dfk","diyob","kkx","or","qniq","qhy","b","jk","rcy"], vec![vec!["eescu","in","hbgw","ardh","ii","om"],vec!["ii","ardh","in","hbgw"],vec!["ardh","rcy"],vec!["ardh","ii","eescu","hbgw","rcy","jk","b"],vec!["hbgw","ii","in","ardh","om"],vec!["gjotw"],vec!["in","ardh","hbgw","om","ii","kkx","qniq"],vec!["om","hbgw","ii","ardh","in","eescu"],vec!["ii","om","hbgw","eescu"],vec!["om","hbgw","in","ardh","eescu"]], vec!["om","hbgw","in","ardh","eescu","ii"])
    ] {
        println!("{:?}", find_all_recipes(i.0.iter().map(|x| x.to_string()).collect::<Vec<String>>(), i.1.iter().map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>(), i.2.iter().map(|x| x.to_string()).collect::<Vec<String>>()));
    }
}

pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    let mut supply_map: HashSet<String> = HashSet::from_iter(supplies);

    for (index, i) in recipes.iter().enumerate() {
        map.insert(i.clone(), ingredients[index].clone());
    }

    pub fn resolve(r: String, i: &HashMap<String, Vec<String>>, supplies: &mut HashSet<String>, traversed: &mut HashSet<String>) -> bool {
        match i.get(&r) {
            Some(ingredients) => {
                for ingredient in ingredients {
                    if !supplies.contains(ingredient) {
                        if traversed.contains(ingredient) {
                            return false;
                        }
                        traversed.insert(ingredient.clone());

                        if !resolve(ingredient.clone(), &i, supplies, traversed) {
                            return false;
                        }
                    }
                }
                supplies.insert(r);
                true
            },
            None => {
                false
            }
        }
    }

    let mut result = vec![];

    for (k, _) in &map {
        if resolve(k.clone(), &map, &mut supply_map, &mut HashSet::new()) {
            result.push(k.clone());
        }
    }

    result
}