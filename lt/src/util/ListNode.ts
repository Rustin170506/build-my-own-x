export class ListNode {
    constructor(
        public val = 0,
        public next: ListNode | null = null,
    ) {}
}

export function createLinkedList(arr: number[]): ListNode | null {
    if (arr.length === 0) return null;
    const head: ListNode = new ListNode(arr[0]);
    let current = head;
    for (let i = 1; i < arr.length; i++) {
        current.next = new ListNode(arr[i]);
        current = current.next;
    }
    return head;
}

export function convertLinkedListToList(head: ListNode | null): number[] {
    const arr: number[] = [];
    let current = head;
    while (current !== null) {
        arr.push(current.val);
        current = current.next;
    }
    return arr;
}
