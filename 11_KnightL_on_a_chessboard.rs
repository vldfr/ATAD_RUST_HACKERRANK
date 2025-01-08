use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::VecDeque;


/*
 * Complete the 'knightlOnAChessboard' function below.
 *
 * The function is expected to return a 2D_INTEGER_ARRAY.
 * The function accepts INTEGER n as parameter.
 */

fn knightlOnAChessboard(n: i32) -> Vec<Vec<i32>> {
    let mut output = vec![vec![-1; n as usize -1]; n as usize -1];
    for i in 0..=n-2{
        for j in 0..=n-2{
            output[i as usize][j as usize] = knightl_shortest_path(n,i+1,j+1);
        }
    }
    output
}

fn knightl_shortest_path(n: i32, a: i32, b: i32) -> i32 {
    // Possible moves for the knightm, pairs of x,y sum components
    let moves = [
        (a as i32, b as i32), (a as i32, -(b as i32)),
        (-(a as i32), b as i32), (-(a as i32), -(b as i32)),
        (b as i32, a as i32), (b as i32, -(a as i32)),
        (-(b as i32), a as i32), (-(b as i32), -(a as i32)),
    ];

    // Queue for BFS and visited set
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n as usize]; n as usize];

    // Start BFS from (0, 0)
    queue.push_back((0, 0, 0)); // (x, y, steps)
    visited[0][0] = true;
    
    // all valid moves are added to the queue and the queue is explored
    // if reaching target we return
    // since each iteration is an additional step, the first result is the best result.
    while let Some((x, y, steps)) = queue.pop_front() {
        // If we reach the target
        if x == n as i32 - 1 && y == n as i32 - 1 {
            return steps;
        }

        // Explore all possible moves
        for &(dx, dy) in &moves {
            let nx = x + dx;
            let ny = y + dy;

            // Check boundaries and if not visited
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 && !visited[nx as usize][ny as usize] {
                visited[nx as usize][ny as usize] = true;
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }

    // If the target is not reachable
    -1
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = knightlOnAChessboard(n);

    for i in 0..result.len() {
        for j in 0..result[i].len() {
            write!(&mut fptr, "{}", result[i][j]).ok();

            if j != result[i].len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
