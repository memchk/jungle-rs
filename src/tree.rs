use generic_array::*;

struct Node<T, N: ArrayLength<NodeRef<T,N>>> {
	value: T,
	parent: NodeParentRef<T,N>,
	children: GenericArray<NodeRef<T, N>, N>
}

struct NodeRef<T, N: ArrayLength<NodeRef<T,N>>> {
	ptr: Option<Box<Node<T,N>>>
}

impl<T, N: ArrayLength<NodeRef<T,N>>> Default for NodeRef<T,N> {
	fn default() -> Self {
		NodeRef {
			ptr: None
		}
	}
}

struct NodeParentRef<T, N: ArrayLength<NodeRef<T,N>>> {
	ptr: Option<*mut Node<T,N>>
}

impl<T, N: ArrayLength<NodeRef<T,N>>> Default for NodeParentRef<T,N> {
	fn default() -> Self {
		NodeParentRef {
			ptr: None
		}
	}
}

struct Tree<T, N: ArrayLength<NodeRef<T,N>>> {
	root: NodeRef<T,N>
}