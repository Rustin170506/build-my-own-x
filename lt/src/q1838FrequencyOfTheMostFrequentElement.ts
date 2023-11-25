export function maxFrequency(nums: number[], k: number): number {
    // Default sort is lexicographical, so we need to provide a comparator.
    nums.sort((a, b) => a - b);
    let [l, r] = [0, 0];
    let [res, total] = [0, 0];

    while (r < nums.length) {
        total += nums[r];

        while (nums[r] * (r - l + 1) > total + k) {
            total -= nums[l];
            l += 1;
        }

        res = Math.max(res, r - l + 1);
        r += 1;
    }

    return res;
}
