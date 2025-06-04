use std::collections::HashMap;

#[rustfmt::skip]
const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

#[rustfmt::skip]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut affected_cells = HashMap::<(i32, i32), u8>::new();

    // Mark cells by how many bombs will blow up on them
    for (y, &row) in minefield.iter().enumerate() {
        for (x, &cell) in row.as_bytes().iter().enumerate() {
            if cell == b'*' {
                for (dy, dx) in OFFSETS.iter() {
                    let entry = affected_cells.entry((y as i32 + dy, x as i32 + dx)).or_insert(0);
                    *entry += 1;
                }
            }
        }
    }

    // Print the cells we care about - ignores the cells that are out of bounds
    minefield.iter().enumerate().map(|(y, row)| {
        row.as_bytes().iter().enumerate().map(|(x, &cell)| {
            if cell == b'*' {
                '*'
            } else {
                match affected_cells.get(&(y as i32, x as i32)).unwrap_or(&0) {
                    0 => ' ',
                    n => (n + b'0') as char,
                }
            }
        }).collect()
    }).collect()
}
