use generic_array::{GenericArray, ArrayLength};
pub use generic_array::typenum;


use std::ptr;
use std::fmt;
use std::mem;

pub struct Node<T, N: ArrayLength<NodeRef<T,N>>> {
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

pub struct NodeRef<T, N: ArrayLength<NodeRef<T,N>>> {
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

pub struct Tree<T, N: ArrayLength<NodeRef<T,N>>> {
	root: NodeRef<T,N>
}

impl<T, N: ArrayLength<NodeRef<T,N>>> Drop for Tree<T,N> {
	fn drop(&mut self) {
		// Traverse to root node
		while self.parent() {}
		
		let mut stk = Vec::new();
		stk.push(self.get_ref());
		while !stk.is_empty() {
			
			//let children = unsafe {(*stk.pop().unwrap().ptr).children.iter()};
			let top = stk.pop().unwrap();
			let children = unsafe {(*top.ptr).children.iter()};

			for child in children {
				if !child.is_null() {
					stk.push(*child);
				}
			}

			unsafe {
				Box::from_raw(top.ptr);
			}
		}
	}
}

impl<T, N: ArrayLength<NodeRef<T,N>>> Tree<T,N> {
	pub fn new(value: T) -> Self {
		Tree {
			root: NodeRef {
				ptr: Box::into_raw(Box::new(Node::new(value)))
			}
		}
	}

	pub fn parent(&mut self) -> bool {
		let parent = unsafe {(*self.root.ptr).parent};
		if parent.is_null() {
			false
		}else {
		    self.root = parent;
		    true
		}
	}

	pub fn child(&mut self, index: usize) -> bool {
		if index >= N::to_usize() {
			return false;
		}
		let child = unsafe {*(*self.root.ptr).children.get_unchecked(index)};
		if child.is_null() {
			false
		} else {
			self.root = child;
			true
		}
	}

	pub fn take(&mut self) -> Tree<T,N> {
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

	pub fn attach(&mut self,tree: Tree<T,N>, index: usize) -> bool {
		if index >= N::to_usize() {
			return false;
		}
		let child = unsafe {*(*self.root.ptr).children.get_unchecked(index)};
		if child.is_null() {
			unsafe {
				(*tree.root.ptr).parent = self.root;
				(*self.root.ptr).children[index] = tree.root;
			};
			mem::forget(tree);
			true
		} else {
			false
		}
	}

	fn get_ref(&self) -> NodeRef<T,N> {
		self.root
	}

	pub fn value(&self) -> &T {
		unsafe {&(*self.root.ptr).value}
	}

	pub fn value_mut(&self) -> &mut T {
		unsafe {&mut (*self.root.ptr).value}
	}
}

#[cfg(test)]
mod tests {
    use tree::Tree;
    use tree::typenum::*;
    use test::Bencher;
    #[test]
    fn create_tree() {
    	let t : Tree<u32,U2> = Tree::new(5);
        assert_eq!(&5, t.value());
    }


    #[test]
    fn check_attach() {
    	let mut t : Tree<u32, U2> = Tree::new(0);
    	t.attach(Tree::new(7),0);
    	t.attach(Tree::new(15),1);
    	let x = *{
    		t.child(0);
    		t.value()
    	};
    	assert_eq!(7, x);
    	t.parent();
    	let x = *{
    		t.child(1);
    		t.value()
    	};
    	assert_eq!(15, x);
    }

    #[test]
    fn large_listtree() {
    	let mut t : Tree<u32, U1> = Tree::new(0);
    	for i in 0..1_000_000 {
    		t.attach(Tree::new(i),0);
    		t.child(0);
    		t.value();
    	}
    }

    #[bench]
    fn test_alloc_attach(b: &mut Bencher) {
   		let mut t : Tree<u32, U1> = Tree::new(0);
   		b.iter(|| {
   			t.attach(Tree::new(0),0);
   			t.child(0);
   		})
    }
    #[bench]
    fn test_alloc_attach_unsafe(b: &mut Bencher) {
   		let mut t : Tree<u32, U1> = Tree::new(0);
   		b.iter(|| {
   			t.attach(Tree::new(0),0);
   			unsafe { t.child_unchecked(0) };
   		})
    }
}