import { insertIntoBST } from "./q0701InsertABinarySearchTree";
import { arrayToBinarySearchTree } from "./util/TreeNode";

test("insertIntoBST", () => {
    expect(insertIntoBST(arrayToBinarySearchTree([4, 2, 7, 1, 3]), 5)).toEqual(
        arrayToBinarySearchTree([4, 2, 7, 1, 3, 5]),
    );
    expect(
        insertIntoBST(
            arrayToBinarySearchTree([40, 20, 60, 10, 30, 50, 70]),
            25,
        ),
    ).toEqual(
        arrayToBinarySearchTree([40, 20, 60, 10, 30, 50, 70, null, null, 25]),
    );
});
