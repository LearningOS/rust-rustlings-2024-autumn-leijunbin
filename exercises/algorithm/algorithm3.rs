/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn order<T:Ord>(array: &mut [T], start: usize, end: usize) -> usize {
    let mut first_index = start;
    
    for index in start + 1..end {
        if array[index] < array[start] {
            first_index += 1;
            array.swap(first_index, index);
        }
    }

    array.swap(start, first_index);

    first_index
}

fn quick_sort<T:Ord>(array: &mut [T], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let dif = order(array, start, end);

    quick_sort(array, start, dif);
    quick_sort(array, dif + 1, end);
}

fn sort<T:Ord>(array: &mut [T]){
	quick_sort(array, 0, array.len());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}