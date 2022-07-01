use std::collections::BTreeMap;

fn main() {
    let test_input_1 = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    let test_input_2 = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
        ";

    let test_input_3 = "\
        fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    let input = "\
FK-gc
gc-start
gc-dw
sp-FN
dw-end
FK-start
dw-gn
AN-gn
yh-gn
yh-start
sp-AN
ik-dw
FK-dw
end-sp
yh-FK
gc-gn
AN-end
dw-AN
gn-sp
gn-FK
sp-FK
yh-gc
        ";

    assert_eq!(count_paths(test_input_1), 10);
    assert_eq!(count_paths(test_input_2), 19);
    assert_eq!(count_paths(test_input_3), 226);
    println!("{}", count_paths(input));

}

fn count_paths(input: &str) -> usize {
    let mut paths = Vec::<Vec<String>>::new();
    let graph = parse_graph(input);

    fn rec(
        graph: &BTreeMap<String, Vec<String>>,
        paths: &mut Vec<Vec<String>>,
        to: &str,
        current_path: Vec<String>,
    ) {
        for child in &graph[to] {
            if current_path.contains(&child.to_string()) && !is_capital(&child) {
                ()
            } else if child == "end" {
                let mut new_path = current_path.clone();
                new_path.push(child.to_string());
                paths.push(new_path);
            } else {
                let mut new_path = current_path.clone();
                new_path.push(child.to_string());
                rec(graph, paths, &child, new_path)
            }
        }
    }

    rec(&graph, &mut paths, "start", vec!["start".to_string()]);

    paths.len()
}

fn is_capital(child: &str) -> bool {
    match child.chars().next() {
        Some(x) => x.is_uppercase(),
        None => {
            println!("{}", child);
            panic!("shit");
        }
    }
}

fn parse_graph(input: &str) -> BTreeMap<String, Vec<String>> {
    let mut graph = BTreeMap::<String, Vec<String>>::new();
    input.trim().lines().for_each(|line| {
        let (x, y) = line.split_once('-').unwrap();
        let v1 = graph.entry(x.to_string()).or_default();
        v1.push(y.to_string());
        let v2 = graph.entry(y.to_string()).or_default();
        v2.push(x.to_string());
    });
    graph
}
