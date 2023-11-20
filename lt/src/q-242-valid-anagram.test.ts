import { isAnagram } from "./q-242-valid-anagram";

test("isAnagram", () => {
  expect(isAnagram("abc", "cba")).toBe(true);
  expect(isAnagram("acc", "c")).toBe(false);
  expect(isAnagram("acc", "cad")).toBe(false);
});
