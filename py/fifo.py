class Node:
    def __init__(self, elem, next):
        self.elem = elem
        self.next = next


# FIFO
class Queue:
    def __init__(self):
        self.head = None

    def push(self, elem):
        new_node = Node(elem=elem, next=self.head)
        self.head = new_node

    def pop(self):
        if self.head:
            old_head = self.head
            self.head = old_head.next
            return old_head.elem
        return None

    def print(self):
        pivo = self.head
        while pivo is not None:
            print(pivo.elem)
            pivo = pivo.next

    def clear(self):
        while self.pop() is not None:
            pass


queue = Queue()

print(queue.pop())
print(queue.push(1))
print(queue.push(2))
print(queue.push(3))
queue.print()
queue.clear()
print(queue.pop())
