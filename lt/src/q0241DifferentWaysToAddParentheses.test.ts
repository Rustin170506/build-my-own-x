import { diffWaysToCompute } from "./q0241DifferentWaysToAddParentheses";

test("diffWaysToCompute", () => {
    expect(diffWaysToCompute("2-1-1").sort()).toStrictEqual([2, 0].sort());
    expect(diffWaysToCompute("2*3-4*5").sort()).toStrictEqual(
        [-34, -14, -10, -10, 10].sort(),
    );
});
