use tower_of_hanoi::{solve_iterative, solve_recursive};

fn is_valid_solution(moves: &[(u32, u32)], n: u32) -> bool {
    let mut pegs: [Vec<u32>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for disk in (1..=n).rev() {
        pegs[0].push(disk);
    }
    for &(from, to) in moves {
        let f = (from - 1) as usize;
        let t = (to - 1) as usize;
        if pegs[f].is_empty() {
            return false;
        }
        let disk = pegs[f].pop().unwrap();
        if let Some(&top) = pegs[t].last() {
            if top < disk {
                return false;
            }
        }
        pegs[t].push(disk);
    }
    pegs[2].len() as u32 == n
}

#[test]
fn compare_solvers_small() {
    for n in 1..=8 {
        let r = solve_recursive(n);
        let it = solve_iterative(n);
        assert_eq!(r.len(), it.len());
        assert!(is_valid_solution(&r, n));
        assert!(is_valid_solution(&it, n));
    }
}
