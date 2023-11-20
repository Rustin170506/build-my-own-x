export function wordPattern(pattern: string, s: string): boolean {
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
