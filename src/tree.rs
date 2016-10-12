use generic_array::*;
use std::ops::Deref;
use std::ptr;

struct Node<T, N: ArrayLength<NodeRef<T,N>>> {
	value: T,
	parent: NodeRef<T,N> ,
	children: GenericArray<NodeRef<T, N>, N>
}

impl<T, N: ArrayLength<NodeRef<T,N>>> Node<T,N> {
	fn new(value: T) -> Self {
		Node {
			value: value,
			parent: NodeRef::default(),
			children: GenericArray::new()
		}
	}
}

struct NodeRef<T, N: ArrayLength<NodeRef<T,N>>> {
	ptr: *mut Node<T,N>
}

impl<T, N: ArrayLength<NodeRef<T,N>>> Copy for NodeRef<T,N> {}
impl<T, N: ArrayLength<NodeRef<T,N>>> Clone for NodeRef<T, N> { fn clone(&self) -> NodeRef<T, N> { *self } }

impl<T, N: ArrayLength<NodeRef<T,N>>> Default for NodeRef<T,N> {
	fn default() -> Self {
		NodeRef {
			ptr: ptr::null_mut()
		}
	}
}

impl<T, N: ArrayLength<NodeRef<T,N>>> NodeRef<T,N> {
	fn is_null(&self) -> bool {
		self.ptr.is_null()
	}
}

struct Tree<T, N: ArrayLength<NodeRef<T,N>>> {
	root: NodeRef<T,N>
}

impl<T,N: ArrayLength<NodeRef<T,N>>> Tree<T,N> {
	fn new(value: T) -> Self {
		Tree {
			root: NodeRef {
				ptr: Box::into_raw(Box::new(Node::new(value)))
			}
		}
	}

	fn parent(&mut self) -> bool {
		let parent = unsafe {(*self.root.ptr).parent};
		if parent.is_null() {
			false
		}else {
		    self.root = parent;
		    true
		}
	}

	fn child(&mut self, index: usize) -> bool {
		let child = unsafe {(*self.root.ptr).children[index]};
		if child.is_null() {
			false
		} else {
			self.root = child;
			true
		}
	}

	fn take(&mut self) -> Tree<T,N> {
		let rtn = Tree {
			root: self.root
		};

		let parent = unsafe {(*self.root.ptr).parent};
		if !parent.is_null() {
			self.root = parent;
		}
		unsafe {(*rtn.root.ptr).parent = NodeRef::default()};
		rtn
	}
}