import { containsDuplicate } from "./q0270ContainsDuplicate";

test("containsDuplicate", () => {
  expect(containsDuplicate([1, 2, 3, 1])).toBe(true);
  expect(containsDuplicate([1, 2, 3, 4])).toBe(false);
  expect(containsDuplicate([1])).toBe(false);
  expect(containsDuplicate([1, 1])).toBe(true);
});
