import { threeSum } from "./q00153ThreeSum";

describe("threeSum ", () => {
    it("should be return [[-1, -1, 2], [-1, 0, 1]]", () => {
        expect(threeSum([-1, 0, 1, 2, -1, -4])).toEqual([
            [-1, -1, 2],
            [-1, 0, 1],
        ]);
    });
    it("should be return []", () => {
        expect(threeSum([])).toEqual([]);
    });
    it("should be return []", () => {
        expect(threeSum([0])).toEqual([]);
    });
    it("should be return []", () => {
        expect(threeSum([0, 0, 0])).toEqual([[0, 0, 0]]);
    });
    it("should be return [[-2, 0, 2]]", () => {
        expect(threeSum([-2, 0, 0, 2, 2])).toEqual([[-2, 0, 2]]);
    });
});
