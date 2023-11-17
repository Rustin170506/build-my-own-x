import { test, expect } from "vitest";

  function wordPattern(pattern: string, s: string): boolean {
    const words = s.split(" ");
    if (pattern.length !== words.length) {
      return false;
    }
    const patternMap = new Map();
    const wordsMap = new Map();

    for (let index = 0; index < pattern.length; index++) {
      const word = words[index];
      const c = pattern.charAt(index);
      if (patternMap.has(c)) {
        const w = patternMap.get(c);
        if (w !== word) {
          return false;
        }
      } else {
        patternMap.set(c, word);
      }
      if (wordsMap.has(word)) {
        const char = wordsMap.get(word);
        if (c !== char) {
          return false;
        }
      } else {
        wordsMap.set(word, c);
      }
    }

    return true;
  }

test("wordPattern", () => {
  expect(wordPattern("abba", "dog cat cat dog")).toBe(true);
  expect(wordPattern("abba", "dog cat cat fish")).toBe(false);
  expect(wordPattern("aaaa", "dog cat cat dog")).toBe(false);
  expect(wordPattern("abba", "dog dog dog dog")).toBe(false);
});
