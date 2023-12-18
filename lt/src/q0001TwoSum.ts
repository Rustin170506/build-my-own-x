export function twoSum(nums: number[], target: number): number[] {
    const numberMap = new Map();

    for (let index = 0; index < nums.length; index++) {
        const complement = target - nums[index];

        if (numberMap.has(complement)) {
            return [numberMap.get(complement), index];
        }

        numberMap.set(nums[index], index);
    }

    throw Error("unreachable");
}
