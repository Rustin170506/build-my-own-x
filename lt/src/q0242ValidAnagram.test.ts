import { isAnagram } from "./q0242ValidAnagram";

test("isAnagram", () => {
  expect(isAnagram("abc", "cba")).toBe(true);
  expect(isAnagram("acc", "c")).toBe(false);
  expect(isAnagram("acc", "cad")).toBe(false);
});
