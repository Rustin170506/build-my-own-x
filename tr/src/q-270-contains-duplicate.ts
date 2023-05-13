import { expect, test } from 'vitest'

function containsDuplicate(nums: number[]): boolean {
    const set = new Set<number>();
    for (const num of nums) {
        if (set.has(num)) {
        return true;
        } else {
        set.add(num);
        }
    }
    return false;
};

test('containsDuplicate', () => {
  expect(containsDuplicate([1, 2, 3, 1])).toBe(true)
})
