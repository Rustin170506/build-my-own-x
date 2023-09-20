import { expect, test } from "vitest";

function containsDuplicate(nums: number[]): boolean {
  if (nums.length == 1) {
    return false;
  }

  const set = new Set<number>();
  for (const num of nums) {
    if (set.has(num)) {
      return true;
    } else {
      set.add(num);
    }
  }
  return false;
}

test("containsDuplicate", () => {
  expect(containsDuplicate([1, 2, 3, 1])).toBe(true);
  expect(containsDuplicate([1, 2, 3, 4])).toBe(false);
  expect(containsDuplicate([1])).toBe(false);
  expect(containsDuplicate([1, 1])).toBe(true);
});
