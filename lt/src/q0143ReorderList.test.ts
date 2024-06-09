import { reorderList } from "./q0143ReorderList";
import { convertLinkedListToList, createLinkedList } from "./util/ListNode";

test("reorderList", () => {
    let list = createLinkedList([1, 2, 3, 4]);
    reorderList(list);
    expect(convertLinkedListToList(list)).toEqual([1, 4, 2, 3]);

    list = createLinkedList([1, 2, 3, 4, 5]);
    reorderList(list);
    expect(convertLinkedListToList(list)).toEqual([1, 5, 2, 4, 3]);

    list = createLinkedList([]);
    reorderList(list);
    expect(convertLinkedListToList(list)).toEqual([]);
});
