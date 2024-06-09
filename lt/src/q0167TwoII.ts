export function twoSum(numbers: number[], target: number): number[] {
    const result = [];
    let [i, j] = [0, numbers.length - 1];
    while (i < j) {
        if (numbers[i] + numbers[j] < target) {
            i++;
        } else if (numbers[i] + numbers[j] > target) {
            j--;
        } else {
            break;
        }
    }

    result.push(i + 1);
    result.push(j + 1);

    return result;
}
