use generic_array::*;
use std::ops::Deref;
use std::ptr;

struct NodePointer<T,N>(*mut Node<T,N>) where N: ArrayLength<NodePointer<T, N>>;
impl<T,N> Copy for NodePointer<T,N> where N: ArrayLength<NodePointer<T, N>> {}
impl<T,N> Clone for NodePointer<T,N> where N: ArrayLength<NodePointer<T, N>> { fn clone(&self) -> NodePointer<T,N> { *self } }

impl<T,N> Default for NodePointer<T,N> 
	where N: ArrayLength<NodePointer<T, N>> {
	fn default() -> Self {
		NodePointer(ptr::null_mut())
	}
}

impl<T,N> Deref for NodePointer<T,N>
	where N: ArrayLength<NodePointer<T, N>> {
	type Target = Node<T,N>;

	fn deref(&self) -> &Node<T,N> {
		unsafe { &*self.0 }
	}
}

struct Node<T,N: ArrayLength<NodePointer<T,N>>> {
	value: T,
	parent: NodePointer<T,N>,
	children: GenericArray<NodePointer<T,N>,N>
}

impl<T,N> Node<T,N>
	where N: ArrayLength<NodePointer<T, N>> {
    fn new(value: T) -> Self{
    	Node {
    		value: value,
    		parent: NodePointer::default(),
    		children: GenericArray::new()
    	}
    }
}

struct Tree<T,N: ArrayLength<NodePointer<T,N>>>(NodePointer<T,N>);

impl<T,N> Tree<T,N>
	where N: ArrayLength<NodePointer<T, N>> {

	fn new(value: T) -> Self {
	    Tree(NodePointer(Box::into_raw(Box::new(Node::new(value)))))
	}

	fn parent(&mut self) -> bool{
		let parent = self.0.parent;
		if parent.0 == ptr::null_mut() {
			false
		} else {
		    self.0 = parent;
		    true
		}
	}

	fn child(&mut self, child: usize) -> bool {
		let child = self.0.children[child];
		if child.0 == ptr::null_mut() {
			false
		} else {
			self.0 = child;
			true
		}
	}
}