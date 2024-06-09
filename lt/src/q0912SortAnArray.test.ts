import { sortArray } from "./q0912SortAnArray";

test("sortArray", () => {
    expect(sortArray([5, 2, 3, 1])).toEqual([1, 2, 3, 5]);
    expect(sortArray([5, 1, 1, 2, 0, 0])).toEqual([0, 0, 1, 1, 2, 5]);
});
