export function singleNonDuplicate(nums: number[]): number {
    let [l, r] = [0, nums.length - 1];
    while (l <= r) {
        const mid = Math.floor(l + (r - l) / 2);
        if (
            (mid - 1 < 0 || nums[mid - 1] !== nums[mid]) &&
            (mid + 1 > nums.length - 1 || nums[mid + 1] !== nums[mid])
        ) {
            return nums[mid];
        }

        const leftSize = nums[mid - 1] === nums[mid] ? mid - 1 : mid;
        if (leftSize % 2 === 0) {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    throw Error("unreachable");
}
