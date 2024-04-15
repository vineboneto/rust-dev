class Node:
    def __init__(self, elem, next=None, prev=None):
        self.elem = elem
        self.next = next
        self.prev = prev


class DoubleList:
    def __init__(self):
        self.head = None
        self.tail = None

    def push_front(self, elem):
        new_node = Node(elem=elem)
        if self.head:
            self.head.prev = new_node
            new_node.next = self.head
            self.head = new_node
        else:
            self.head = new_node
            self.tail = new_node

    def pop_front(self):
        if self.head:
            old_head = self.head
            self.head = self.head.next
            if self.head:
                self.head.prev = None
            else:
                self.tail = None
            return old_head.elem

        return None

    def push_back(self, elem):
        new_node = Node(elem=elem)
        if self.tail:
            self.tail.next = new_node
            new_node.prev = self.tail
            self.tail = new_node
        else:
            self.head = new_node
            self.tail = new_node

    def pop_back(self):
        if self.tail:
            old_tail = self.tail
            self.tail = self.tail.prev
            if self.tail:
                self.tail.next = None
            else:
                self.head = None
            return old_tail.elem

        return None

    def print_front(self):
        pivo = self.head
        while pivo is not None:
            print(pivo.elem)

            pivo = pivo.next

    def print_back(self):
        pivo = self.tail
        while pivo is not None:
            print(pivo.elem)
            pivo = pivo.prev

    def clear(self):
        while self.pop_front() is not None:
            pass


list = DoubleList()

list.push_back(1)
list.push_back(2)
list.print_back()
list.clear()
