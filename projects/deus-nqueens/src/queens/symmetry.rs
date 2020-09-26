use super::*;

impl NQueensState {
    /// all points under rotation and mirror symmetry
    pub fn symmetry_point(&self, x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
        let n = self.rank;
        from_generator(move || {
            yield (x, y);
            yield (x, n - y - 1);
            yield (n - x - 1, y);
            yield (n - x - 1, n - y - 1);
            yield (y, x);
            yield (y, n - x - 1);
            yield (n - y - 1, x);
            yield (n - y - 1, n - x - 1);
        })
        // .filter(move |(x, y)| *x >= 0 && *x < n && *y >= 0 && *y < n)
        // .map(|(x, y)| (x as usize, y as usize))
    }
    // all symmetry points that filled on the column
    pub fn symmetry_banned(&self, column: isize) -> BTreeSet<isize> {
        let mut out = BTreeSet::new();
        for (sx, sy) in self.filled.iter().enumerate() {
            for (x, y) in self.symmetry_point(sx as isize, *sy) {
                if x == column {
                    out.insert(y);
                }
            }
        }
        out
    }
    pub fn symmetry_available_moves(&self) -> Vec<isize> {
        let column = self.filled.len();
        let banned = self.symmetry_banned(column as isize);
        self.unused.iter().filter(move |x| !banned.contains(x)).copied().collect()
    }
}
