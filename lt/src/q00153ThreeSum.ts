export function threeSum(nums: number[]): number[][] {
    const result: number[][] = [];
    nums.sort((a, b) => a - b);
    nums.forEach((number, index) => {
        if (index > 0 && nums[index - 1] === nums[index]) {
            return;
        }
        let start = index + 1;
        let end = nums.length - 1;
        while (start < end) {
            const sum = number + nums[start] + nums[end];
            if (sum === 0) {
                result.push([nums[index], nums[start], nums[end]]);
                while (start < end && nums[start] === nums[start + 1]) {
                    start += 1;
                }
                while (start < end && nums[end - 1] == nums[end]) {
                    end -= 1;
                }
                start += 1;
                end -= 1;
            } else if (sum < 0) {
                start += 1;
            } else {
                end -= 1;
            }
        }
    });
    return result;
}
