use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let test_input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    assert_eq!(solution_a(test_input).unwrap(), 40);
    assert_eq!(solution_b(test_input).unwrap(), 315);

    let input = include_str!("inputs.txt");
    println!("{}", solution_a(input).unwrap());
    println!("{}", solution_b(input).unwrap());
}

fn solution_a(input: &str) -> Option<u32> {
    let matrix = parse_matrix(input);
    let start = Position { row: 0, col: 0 };
    let goal = Position {
        row: matrix.len() - 1,
        col: matrix[0].len() - 1,
    };
    let adjacency_graph = matrx_to_adjacency_graph(matrix);
    dijkstra_shortest_path(adjacency_graph, start, goal)
}

fn solution_b(input: &str) -> Option<u32> {
    let matrix = parse_matrix(input);
    let matrix = extend_matrix(matrix);

    let start = Position { row: 0, col: 0 };
    let goal = Position {
        row: matrix.len() - 1,
        col: matrix[0].len() - 1,
    };

    let adjacency_graph = matrx_to_adjacency_graph(matrix);
    dijkstra_shortest_path(adjacency_graph, start, goal)
}

fn extend_matrix(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    fn increment(cost: u32, i: u32) -> u32 {
        if cost + i > 9 {
            (cost + i) % 9
        } else {
            cost + i
        }
    }

    // extend vertically
    let mut vert = matrix.clone();
    for i in 1..u32::try_from(5).unwrap() {
        vert.extend(
            matrix
                .iter()
                .map(|row| row.iter().map(|cost| increment(*cost, i)).collect()),
        );
    }

    // extend horizontally
    let mut hori = vert.clone();
    for inc in 1..u32::try_from(5).unwrap() {
        for (i, row) in hori.iter_mut().enumerate() {
            row.extend(vert[i].iter().map(|cost| increment(*cost, inc)));
        }
    }

    hori
}

fn matrx_to_adjacency_graph(matrix: Vec<Vec<u32>>) -> HashMap<Position, Vec<Edge>> {
    let mut graph = HashMap::new();

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let mut neighbours = Vec::new();

            if row > 0 {
                neighbours.push(Edge {
                    position: Position { row: row - 1, col },
                    cost: matrix[row - 1][col],
                });
            }

            if row < matrix.len() - 1 {
                neighbours.push(Edge {
                    position: Position { row: row + 1, col },
                    cost: matrix[row + 1][col],
                });
            }

            if col > 0 {
                neighbours.push(Edge {
                    position: Position { row, col: col - 1 },
                    cost: matrix[row][col - 1],
                });
            }

            if col < matrix[row].len() - 1 {
                neighbours.push(Edge {
                    position: Position { row, col: col + 1 },
                    cost: matrix[row][col + 1],
                });
            }

            graph.insert(Position { row, col }, neighbours);
        }
    }

    graph
}

fn parse_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct State {
    cost: u32,
    position: Position,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    // This is a min-heap, so we want the smallest element on top
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    position: Position,
    cost: u32,
}

fn dijkstra_shortest_path(
    graph: HashMap<Position, Vec<Edge>>,
    start: Position,
    goal: Position,
) -> Option<u32> {
    let mut dist: HashMap<Position, u32> =
        HashMap::from_iter(graph.keys().map(|position| (*position, std::u32::MAX)));
    let mut heap = BinaryHeap::<State>::new();

    // we're at `start`, with a zero cost
    dist.entry(start).and_modify(|d| *d = 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        for edge in &graph[&position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.position,
            };

            if next.cost < dist[&next.position] {
                heap.push(next);
                dist.entry(next.position).and_modify(|d| *d = next.cost);
            }
        }
    }

    // Goal not reachable
    None
}
