use std::{fs::read_to_string, collections::BinaryHeap, cmp::Ordering};

fn dijkstra(start: (usize, usize), end: (usize, usize), raw_input: &Vec<Vec<char>>) -> u32 {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct DijkstraData {
        position: (usize, usize),
        previous: Option<(usize, usize)>,
        cost: u32
    }

    impl DijkstraData {
        fn new(position: (usize, usize)) -> DijkstraData {
            DijkstraData { position, previous: Option::None, cost: u32::MAX }
        }

        fn new_start(position: (usize, usize)) -> DijkstraData {
            DijkstraData { position, previous: Option::None, cost: 0 }
        }
    }

    impl Ord for DijkstraData {
        fn cmp(&self, other: &Self) -> Ordering {
            // Notice that the we flip the ordering on costs.
            // In case of a tie we compare positions - this step is necessary
            // to make implementations of `PartialEq` and `Ord` consistent.
            other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
        }
    }
    
    // `PartialOrd` needs to be implemented as well.
    impl PartialOrd for DijkstraData {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let (sx, sy) = start;
    let (ex, ey) = end;

    let mut dijkstra_data : Vec<Vec<DijkstraData>> = Vec::new();
    let mut queue: BinaryHeap<DijkstraData> = BinaryHeap::new();
    for (y, row) in raw_input.iter().enumerate() {
        dijkstra_data.push(Vec::with_capacity(raw_input[0].len()));
        for (x, _) in row.iter().enumerate() {
            if x == start.0 && y == start.1 {
                dijkstra_data[y].push(DijkstraData::new_start((x, y)));
                queue.push(DijkstraData::new_start((x, y)));
            } else {
                queue.push(DijkstraData::new((x, y)));
                dijkstra_data[y].push(DijkstraData::new((x, y)));
            } 
        }
    }

    queue.push(dijkstra_data[sy][sx]);

    while let Some(node) = queue.pop() {
        if node.position == (ex, ey) { break; }

        // Important as we may have already found a better way
        if node.cost > dijkstra_data[node.position.1][node.position.0].cost { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        // The "as u64"s are needed for some reason since there are overflows otherwise
        let (x,y) = node.position;
        if x > 0 && (raw_input[y][x - 1] as u32) - 1 <= raw_input[y][x] as u32 {
            if dijkstra_data[y][x-1].cost as u64 > node.cost as u64 + 1 {
                dijkstra_data[y][x-1].cost = node.cost + 1;
                dijkstra_data[y][x-1].previous = Some((x,y));
                queue.push(dijkstra_data[y][x-1]);
            }
        }
        if x < raw_input[0].len() - 1 && (raw_input[y][x + 1] as u32) - 1 <= raw_input[y][x] as u32 {
            if dijkstra_data[y][x+1].cost as u64 > node.cost as u64 + 1 {
                dijkstra_data[y][x+1].cost = node.cost + 1;
                dijkstra_data[y][x+1].previous = Some((x,y));
                queue.push(dijkstra_data[y][x+1]);
            }
        }
        if y > 0 && (raw_input[y - 1][x] as u32) - 1 <= raw_input[y][x] as u32 {
            if dijkstra_data[y-1][x].cost as u64 > node.cost as u64 + 1 {
                dijkstra_data[y-1][x].cost = node.cost + 1;
                dijkstra_data[y-1][x].previous = Some((x,y));
                queue.push(dijkstra_data[y-1][x]);
            }
        }
        if y < raw_input.len() - 1 && (raw_input[y + 1][x] as u32) - 1 <= raw_input[y][x] as u32 {
            if dijkstra_data[y+1][x].cost as u64 > node.cost as u64 + 1 {
                dijkstra_data[y+1][x].cost = node.cost + 1;
                dijkstra_data[y+1][x].previous = Some((x,y));
                queue.push(dijkstra_data[y+1][x]);
            }
        }
    }
    dijkstra_data[end.1][end.0].cost
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let mut raw_input = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

        let (mut sx, mut sy) = (0, 0);
        let (mut ex, mut ey) = (0, 0);
    for (y, row) in raw_input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                sx = x;
                sy = y;
            } else {
                if c == 'E' {
                    ex = x;
                    ey = y;
                }
            }
        }
    }
    raw_input[sy][sx] = 'a';
    raw_input[ey][ex] = 'z';

    let result1 = dijkstra((sx, sy), (ex, ey), &raw_input);

    println!("{}", result1);

    let mut min = u32::MAX;
    for (y, row) in raw_input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'a' {
                let result = dijkstra((x,y), (ex,ey), &raw_input);
                if result < min {
                    min = result;
                }
            }
        }
    }

    println!("{}", min);

}
