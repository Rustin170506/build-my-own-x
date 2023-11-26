import { singleNonDuplicate } from "./q0540SingleElementInASortedArray";

test("singleNonDuplicate", () => {
    // expect(singleNonDuplicate([1, 1, 2, 3, 3])).toBe(2);
    expect(singleNonDuplicate([1, 2, 2, 3, 3])).toBe(1);
    expect(singleNonDuplicate([1, 1, 2, 2, 3])).toBe(3);
    expect(singleNonDuplicate([1])).toBe(1);
    expect(singleNonDuplicate([1, 1, 2, 2, 3, 3, 4])).toBe(4);
    expect(() => singleNonDuplicate([])).toThrow("unreachable");
});
