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
