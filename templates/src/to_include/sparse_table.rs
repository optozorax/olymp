// TODO RENAME SPARSE TABLE AND MADE IT WORK FOR ALL IDEMPOTENT FUNCTIONS

// https://cp-algorithms.com/data_structures/sparse-table.html

#[derive(Debug, Clone)]
struct Rmq<'a, T> {
    input: &'a [T],
    inner: Vec<Vec<usize>>,
    kind: RmqType,
}

#[derive(Clone, Debug)]
enum RmqType {
    Min,
    Max,
}

impl<'a, T: Ord + Copy> Rmq<'a, T> {
    pub fn create(input: &'a [T], kind: RmqType) -> Self {
        let levels = Self::largest_bit(input.len());
        let mut inner = Vec::with_capacity(levels);
        inner.push((0..input.len()).collect::<Vec<_>>());
        let mut result = Self {
            input,
            inner,
            kind,
        };
        for k in 1..=levels {
            let previous = result.inner.last().unwrap();
            let to_push = (0..(input.len() - (1 << k) + 1))
                .map(|i| result.better_index(previous[i], previous[i + (1 << (k - 1))]))
                .collect::<Vec<_>>();
            result.inner.push(to_push);
        }
        result
    }

    fn largest_bit(a: usize) -> usize {
        (std::mem::size_of::<usize>() * 8 - 1).saturating_sub(a.leading_zeros() as usize)
    }

    fn better_index(&self, a: usize, b: usize) -> usize {
        match self.kind {
            RmqType::Min => if self.input[a] < self.input[b] {
                a
            } else {
                b
            },
            RmqType::Max => if self.input[a] > self.input[b] {
                a
            } else {
                b
            },
        }
    }

    pub fn most_index(&self, on: Range<usize>) -> usize {
        assert!(on.start != on.end);
        let a = on.start;
        let b = on.end;
        let level = Self::largest_bit(b - a);
        self.better_index(self.inner[level][a], self.inner[level][b - (1 << level)])
    }

    pub fn most_for_segment(&self, on: Range<usize>) -> T {
        assert!(on.start != on.end);
        self.input[self.most_index(on)]
    }
}

fn find_max_segment_with_this_number_as_min<T: Ord, F: Fn(Range<usize>) -> T>(
    on: Range<usize>,
    elem: T,
    elem_pos: usize,
    min_for_segment: F,
) -> Range<usize> {
    let left_pos = binary_search(on.start..elem_pos, |pos| {
        min_for_segment(pos..elem_pos) >= elem
    })
    .unwrap_or(elem_pos);
    let right_pos = binary_search(elem_pos + 1..on.end, |pos| {
        min_for_segment(elem_pos + 1..pos + 1) < elem
    })
    .unwrap_or(on.end);
    left_pos..right_pos
}

fn find_max_segment_with_this_number_as_max<T: Ord, F: Fn(Range<usize>) -> T>(
    on: Range<usize>,
    elem: T,
    elem_pos: usize,
    max_for_segment: F,
) -> Range<usize> {
    let left_pos = binary_search(on.start..elem_pos, |pos| {
        max_for_segment(pos..elem_pos) <= elem
    })
    .unwrap_or(elem_pos);
    let right_pos = binary_search(elem_pos + 1..on.end, |pos| {
        max_for_segment(elem_pos + 1..pos + 1) > elem
    })
    .unwrap_or(on.end);
    left_pos..right_pos
}
