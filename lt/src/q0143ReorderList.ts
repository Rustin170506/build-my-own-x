import { ListNode } from "./util/ListNode";

export function reorderList(head: ListNode | null): void {
    if (!head) return;

    // Find the middle of the list.
    let slow = head,
        fast = head;
    while (fast.next && fast.next.next) {
        slow = slow.next!;
        fast = fast.next.next;
    }

    // Split the list into two halves.
    let secondHalf = slow.next;
    slow.next = null;

    // Reverse the second half.
    let prev = null,
        current = secondHalf;
    while (current) {
        const next = current.next;
        current.next = prev;
        prev = current;
        current = next;
    }
    secondHalf = prev;

    // Merge the two halves.
    let firstHalf: ListNode | null = head;
    while (firstHalf && secondHalf) {
        const nextFirstHalf: ListNode | null = firstHalf.next;
        const nextSecondHalf = secondHalf.next;

        firstHalf.next = secondHalf;
        secondHalf.next = nextFirstHalf;

        firstHalf = nextFirstHalf;
        secondHalf = nextSecondHalf;
    }
}
