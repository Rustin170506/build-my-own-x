export function productExceptSelf(nums: number[]): number[] {
    const prefix = new Array(nums.length).fill(1);
    const postfix = new Array(nums.length).fill(1);

    for (let index = 1; index < nums.length; index++) {
        prefix[index] = prefix[index - 1] * nums[index - 1];
    }

    for (let index = nums.length - 2; index >= 0; index--) {
        postfix[index] = postfix[index + 1] * nums[index + 1];
    }

    const result = new Array(nums.length).fill(0);
    for (let index = 0; index < nums.length; index++) {
        const product = prefix[index] * postfix[index];
        result[index] = Object.is(product, -0) ? 0 : product;
    }
    return result;
}

export function productExceptSelf1(nums: number[]): number[] {
    const result = new Array(nums.length).fill(1);
    let prefix = 1;
    for (let index = 0; index < nums.length; index++) {
        result[index] = prefix;
        prefix *= nums[index];
    }

    let postfix = 1;
    for (let index = nums.length - 1; index >= 0; index--) {
        result[index] *= postfix;
        postfix *= nums[index];
    }

    return result;
}
