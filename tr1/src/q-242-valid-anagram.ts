import { expect, test } from "vitest";

function isAnagram(s: string, t: string): boolean {
  if (s.length != t.length) {
    return false;
  }

  const s_map = new Map<string, number>();

  s.split("").forEach((c) => {
    if (s_map.has(c)) {
      s_map.set(c, s_map.get(c)! + 1);
    } else {
      s_map.set(c, 1);
    }
  });

  t.split("").forEach((c) => {
    if (s_map.has(c)) {
      const v = s_map.get(c)!;
      if (v == 1) {
        s_map.delete(c);
      } else {
        s_map.set(c, v - 1);
      }
    }
  });

  return s_map.size == 0;
}

test("isAnagram", () => {
  expect(isAnagram("abc", "cba")).toBe(true);
  expect(isAnagram("acc", "c")).toBe(false);
  expect(isAnagram("acc", "cad")).toBe(false);
});
