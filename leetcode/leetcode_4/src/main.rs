use rand::{Rng, SeedableRng, StdRng};
use std::time::{Instant};

use std::i32::MAX;

pub struct Solution {}

type Pointer = (usize, usize);

fn count_from_start(p: &Pointer) -> usize {
	p.0 + p.1
}

/* 
	Получает на вход отсортированный массив
	Находит такое положение указателя, которое:
		Слева от него находится число меньше value
		Справа от него число больше или равное value
	При этом положение указателя 0 считается таким, что у него нет левого, а правый - это первый элемент
*/
fn normalize(mas: &Vec<i32>, start: &usize, end: &usize, value: i32) -> usize {
	let mut start = start.clone();
	let mut end = end.clone();
	if start >= mas.len() {
		return start;
	}
	loop {
		//println!("{:?}, {:?}", start, end);
		if mas[start] >= value {
			return start;
		}
		if end - start == 0 {
			return start;
		}
		if end - start == 1 {
			return end;
		}
		
		let center = (start + end) / 2;
		if mas[center] < value {
			start = center;
		} else {
			end = center;
		}
	}
}

impl Solution {
	pub fn find_median_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
		/* 
		Создаём два указателя - на начало и на конец массива
			Двигаем указатель в том массиве, где больше всего элементов между началом и концом на середину
			Сдвигаем указатель так, чтобы он стал нормализованным
			Смотрим сколько от начала количество элементов
				Если больше середины то возвращаем старый указатель в начало и ставим текущий указатель как указатель в конец
				Если меньше, то наоборот
				Если ровно половина, то мы нашли то, что надо

		*/
		//println!("\nnums1: {:?}, nums2: {:?}", nums1, nums2);
		if nums1.len() == 0 {
			return median(&nums2);
		}
		if nums2.len() == 0 {
			return median(&nums1);
		}
		let mut start: Pointer = (0, 0);
		let mut end: Pointer = (nums1.len(), nums2.len());

		let center_pos = (nums1.len() + nums2.len() - 1) / 2;

		//println!("start: {:?} end: {:?}", start, end);

		let mut last_center = start;
		loop {
			let mut center: Pointer = start;
			if (end.0 - start.0) > (end.1 - start.1) {
				center.0 = (start.0 + end.0) / 2;
				center.1 = normalize(&nums2, &start.1, &end.1, nums1[center.0]);
			} else {
				center.1 = (start.1 + end.1) / 2;
				center.0 = normalize(&nums1, &start.0, &end.0, nums2[center.1]);
			}

			if start == end {
				break;
			}

			//println!("start: {:?} end: {:?} center: {:?}", start, end, center);

			//println!("count_from_start(center): {:?}, center_pos: {:?}", count_from_start(&center), center_pos);
			if count_from_start(&center) <= center_pos {
				start = center.clone();
			} else {
				end = center.clone();
			}

			if last_center == center {
				break;
			}
			last_center = center;
		}

		/*if count_from_start(&start) + 2 <= center_pos {
			start = end.clone();
		}*/
		//println!("count_from_start(start): {:?}, center_pos: {:?}", count_from_start(&start), center_pos);

		//println!("start: {:?} end: {:?}", start, end);

		let mut array = vec![];
		if nums1.len() > start.0 {
			array.push(nums1[start.0]);
		}
		if nums2.len() > start.1 {
			array.push(nums2[start.1]);
		}
		if nums1.len() > start.0 + 1 {
			array.push(nums1[start.0 + 1]);
		}
		if nums2.len() > start.1 + 1 {
			array.push(nums2[start.1 + 1]);
		}
		array.sort();

		let offset = center_pos - count_from_start(&start);

		//println!("array: {:?}, offset: {:?}", array, offset);

		if (nums1.len() + nums2.len()) % 2 == 0 {
			return (array[0 + offset] + array[1 + offset]) as f64 / 2.0;
		} else {
			return array[0 + offset] as f64;
		}
		
	}
}

fn median(nums: &Vec<i32>) -> f64 {
	if nums.len() % 2 == 0 {
		return (nums[(nums.len()-1)/2] + nums[(nums.len()-1)/2 + 1]) as f64 / 2.0;
	} else {
		return nums[nums.len()/2] as f64;
	}
}

fn median_stupid(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
	/*let mut c = nums1.clone();
	c.extend(nums2.clone());
	c.sort();
	return median(&c);*/
	let (mut i, mut j) = (0, 0);
    let total_length = nums1.len() + nums2.len();
    let median = total_length / 2;
    let mut cur = 0;
    let mut last = 0;
    while i + j <= median {
        let num1 = nums1.get(i).or(Some(&MAX)).unwrap();
        let num2 = nums2.get(j).or(Some(&MAX)).unwrap();
        if num1 < num2 {
            i += 1;
            last = cur;
            cur = *num1;
        } else {
            j += 1;
            last = cur;
            cur = *num2;
        }
    }
    
    if total_length % 2 == 0 {
        return (last + cur) as f64 / 2.0;
    }

    cur as f64
}

fn test_medians(a: Vec<i32>, b: Vec<i32>) {
	let slow = median_stupid(&a, &b);
	let fast = Solution::find_median_sorted_arrays(&a, &b);
	assert_eq!(fast, slow);
}

fn main() {
	assert_eq!(normalize(&vec![1, 2, 3, 4], &0, &4, 0), 0);
	assert_eq!(normalize(&vec![1, 2, 3, 4], &0, &4, 1), 0);
	assert_eq!(normalize(&vec![1, 2, 3, 4], &0, &4, 2), 1);
	assert_eq!(normalize(&vec![1, 2, 3, 4], &0, &4, 3), 2);
	assert_eq!(normalize(&vec![1, 2, 3, 4], &0, &4, 4), 3);
	assert_eq!(normalize(&vec![1, 2, 3, 4], &0, &4, 5), 4);
	assert_eq!(normalize(&vec![1, 3, 5, 7], &0, &4, 4), 2);
	assert_eq!(normalize(&vec![1, 3], &0, &2, 2), 1);
	assert_eq!(normalize(&vec![1], &0, &1, 2), 1);
	assert_eq!(normalize(&vec![1], &0, &1, 1), 0);

	test_medians(vec![1, 3], vec![2]);
	test_medians(vec![1, 3, 5, 7], vec![2, 4, 6]);

	test_medians(vec![1, 2], vec![3, 4]);
	test_medians(vec![1], vec![3, 4]);
	test_medians(vec![1], vec![2, 3, 4]);

	test_medians(vec![1], vec![1]);
	test_medians(vec![1, 2], vec![1, 2]);
	test_medians(vec![1, 2, 3], vec![1, 2, 3]);

	test_medians(vec![1], vec![]);
	test_medians(vec![1, 2], vec![]);
	test_medians(vec![1, 2, 3], vec![]);

	test_medians(vec![1, 3, 4, 7, 9], vec![0, 2, 5, 6]);
	test_medians(vec![1, 5, 5, 5, 9], vec![0, 5, 6]);

	test_medians(vec![1, 3, 4, 7, 9], vec![1, 2]);

	test_medians(vec![3, 4, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23], vec![1, 2]);

	let elements_count = 100000000;
	println!("Elements: {:?}", elements_count);

	let seed: &[_] = &[1, 2, 3, 4];
	let mut rng: StdRng = SeedableRng::from_seed(seed);

	let mut a: Vec<i32> = Vec::with_capacity(elements_count);
	let mut b: Vec<i32> = Vec::with_capacity(elements_count);
	for _ in 0..elements_count {
		a.push(rng.gen::<i16>().into());
		b.push(rng.gen::<i16>().into());
	}
	a.sort();
	b.sort();

	println!("Arrays created");

	let start = Instant::now();
    let result_my = Solution::find_median_sorted_arrays(&a, &b);
    let duration_my = start.elapsed();

    println!("my: {:?}, {:?}", duration_my, result_my);

    let start_stupid = Instant::now();
    let result_stupid = median_stupid(&a, &b);
    let duration_stupid = start_stupid.elapsed();

    println!("stupid: {:?}, {:?}", duration_stupid, result_stupid);
}
