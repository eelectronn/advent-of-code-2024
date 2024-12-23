use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let vertex = line.split_once("-").unwrap();
            (vertex.0, vertex.1)
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<(&str, &str)>) -> usize {
    // println!("{:?}", input);
    let edges: HashSet<(&str, &str)> = HashSet::from_iter(input.iter().cloned());
    let rev_edges: HashSet<(&str, &str)> =
        HashSet::from_iter(input.iter().map(|(v1, v2)| (*v2, *v1)));
    let edges = edges
        .union(&rev_edges)
        .cloned()
        .collect::<HashSet<(&str, &str)>>();

    let mut vertices = HashSet::new();
    for (v1, v2) in input {
        vertices.insert(*v1);
        vertices.insert(*v2);
    }

    let mut triagles = HashSet::new();

    for (v1, v2) in &edges {
        for v3 in &vertices {
            if edges.contains(&(v1, v3)) && edges.contains(&(v2, v3)) {
                let mut vertices = vec![*v1, *v2, *v3];
                vertices.sort();
                triagles.insert((vertices[0], vertices[1], vertices[2]));
            }
        }
    }

    let mut triagles: Vec<(&str, &str, &str)> = triagles.iter().cloned().collect();
    triagles.sort();

    let mut count = 0;
    for (v1, v2, v3) in &triagles {
        if v1.chars().next().unwrap() == 't'
            || v2.chars().next().unwrap() == 't'
            || v3.chars().next().unwrap() == 't'
        {
            count += 1;
        }
    }

    count
}

pub fn solve_part_2(input: &Vec<(&str, &str)>) -> String {
    let mut neighbourhood: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (v1, v2) in input {
        neighbourhood.entry(v1).or_insert(HashSet::new()).insert(v2);
        neighbourhood.entry(v2).or_insert(HashSet::new()).insert(v1);
    }

    let r = HashSet::new();
    let p: HashSet<&str> = neighbourhood.keys().cloned().collect();
    let x = HashSet::new();
    let mut maximal_cliques = Vec::new();
    bron_kerbosch(&r, &p, &x, &neighbourhood, &mut maximal_cliques);
    maximal_cliques.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut larget_clique = Vec::from_iter(maximal_cliques[0].iter().cloned());
    larget_clique.sort();
    larget_clique.join(",")
}

pub fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &HashSet<&'a str>,
    x: &HashSet<&'a str>,
    neighbourhood: &HashMap<&'a str, HashSet<&'a str>>,
    maximal_cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        maximal_cliques.push(r.clone());
    }
    let p_iter = p.into_iter();
    let mut p = p.clone();
    let mut x = x.clone();
    for v in p_iter {
        let mut r_copy = r.clone();
        r_copy.insert(v);
        let mut p_copy = p.clone();
        p_copy.retain(|x| neighbourhood.get(v).unwrap().contains(x));
        let mut x_copy = x.clone();
        x_copy.retain(|x| neighbourhood.get(v).unwrap().contains(x));
        bron_kerbosch(&r_copy, &p_copy, &x_copy, neighbourhood, maximal_cliques);
        p.remove(v);
        x.insert(v);
    }
}
