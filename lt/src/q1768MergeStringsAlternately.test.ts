import { mergeAlternately } from "./q1768MergeStringsAlternately";

test("mergeAlternately", () => {
    expect(mergeAlternately("abc", "pqr")).toBe("apbqcr");
    expect(mergeAlternately("ab", "pqrs")).toBe("apbqrs");
    expect(mergeAlternately("abcd", "pq")).toBe("apbqcd");
    expect(mergeAlternately("a", "pq")).toBe("apq");
});
