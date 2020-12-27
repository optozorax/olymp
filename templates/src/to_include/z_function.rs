fn z_function<T: PartialEq>(input: &[T]) -> Vec<usize> {
    let mut z = vec![0; input.len()];
    let mut l = 0usize;
    let mut r = 0usize;
    for i in 1..input.len() {
        let prototype_z = z[i - l];
        let zi = &mut z[i];
        if let Some(dist_to_end) = (r + 1).checked_sub(i) {
            *zi = min(prototype_z, dist_to_end);
        }
        while *zi+i < input.len() && input[*zi] == input[*zi + i] {
            *zi += 1;
        }
        let newl = i;
        let newr = *zi + i - 1;
        if newr > r {
            l = newl;
            r = newr;
        }
    }
    z
}

#[derive(Clone, Debug)]
struct ZVec<T> {
    vec: Vec<T>,
    z: Vec<usize>,
}

impl<T: PartialEq> FromIterator<T> for ZVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec: Vec<T> = Vec::from_iter(iter);
        let z = z_function(&vec);
        ZVec {
            vec,
            z,
        }
    }
}
