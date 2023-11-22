export function sortArray(nums: number[]): number[] {
    mergeSort(nums, 0, nums.length);
    return nums;
}

function mergeSort(nums: number[], l: number, r: number) {
    if (r <= l + 1) {
        return;
    }

    const m = l + Math.floor((r - l) / 2);
    mergeSort(nums, l, m);
    mergeSort(nums, m, r);
    merge(nums, l, m, r);
}
function merge(nums: number[], l: number, m: number, r: number) {
    const left = nums.slice(l, m);
    const right = nums.slice(m, r);
    let i = l;
    let j = 0;
    let k = 0;

    while (j < left.length && k < right.length) {
        if (left[j] <= right[k]) {
            nums[i] = left[j];
            j += 1;
        } else {
            nums[i] = right[k];
            k += 1;
        }
        i += 1;
    }

    while (j < left.length) {
        nums[i] = left[j];
        i += 1;
        j += 1;
    }
    while (k < right.length) {
        nums[i] = right[k];
        i += 1;
        k += 1;
    }
}
