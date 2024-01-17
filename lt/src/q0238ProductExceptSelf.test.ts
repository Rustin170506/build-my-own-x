import { productExceptSelf } from "./q0238ProductExceptSelf";
test("productExceptSelf", () => {
    expect(productExceptSelf([1, 2, 3, 4])).toStrictEqual([24, 12, 8, 6]);
    expect(productExceptSelf([-1, 1, 0, -3, 3])).toStrictEqual([0, 0, 9, 0, 0]);
});
