import { maxFrequency } from "./q1838FrequencyOfTheMostFrequentElement";

test("maxFrequency", () => {
    expect(maxFrequency([1, 2, 4], 5)).toBe(3);
    expect(maxFrequency([1, 4, 8, 13], 5)).toBe(2);
    expect(maxFrequency([3, 9, 6], 2)).toBe(1);
});
