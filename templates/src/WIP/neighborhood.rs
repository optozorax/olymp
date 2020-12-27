const MOORE_NEIGHBORHOOD: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const VON_NEUMANN_NEIGHBORHOOD: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn for_each_neighborhood<T, F: FnMut(T)>(
    field: &[Vec<T>]
    (x, y): (usize, usize),
    neighborhood: &[(isize, isize)],
    mut f: F,
) {
    for (of_x, of_y) in neighborhood {
        let new_pos = (|| {
            Some((
                usize::try_from(isize::try_from(x).ok()?.checked_add(*of_x)?).ok()?,
                usize::try_from(isize::try_from(y).ok()?.checked_add(*of_y)?).ok()?,
            ))
        })();
        if let Some((x, y)) = new_pos {
            if x < field.len() && y < field[x].len() {
                f(field[x][y]);
            }
        }
    }
}