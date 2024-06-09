import { topKFrequent, topKFrequent2 } from "./q0349TopKFrequent";

test("topKFrequent", () => {
    expect(topKFrequent([1, 1, 2, 3, 3], 2)).toEqual([1, 3]);
    expect(topKFrequent([1, 2, 2, 3, 3], 2)).toEqual([2, 3]);
    expect(topKFrequent([1, 1, 2, 2, 3], 2)).toEqual([1, 2]);
    expect(topKFrequent([1], 1)).toEqual([1]);
    expect(topKFrequent([1, 1, 2, 2, 3, 3, 4], 2)).toEqual([1, 2]);
});

test("topKFrequent2", () => {
    expect(topKFrequent2([1, 1, 2, 3, 3], 2)).toEqual([1, 3]);
    expect(topKFrequent2([1, 2, 2, 3, 3], 2)).toEqual([2, 3]);
    expect(topKFrequent2([1, 1, 2, 2, 3], 2)).toEqual([1, 2]);
    expect(topKFrequent2([1], 1)).toEqual([1]);
    expect(topKFrequent2([1, 1, 2, 2, 3, 3, 4], 2)).toEqual([1, 2]);
});
