use algorithms_exercises::node::Node;

#[test]
pub fn new_node_test(){
    let node = Node::new(46); 
    assert_eq!(Node{ value : 46, next : None, }, node);

}


#[test]
pub fn node_to_string(){
    let node = Node::new(46); 
    assert_eq!(String::from("Node value : 46"), node.to_string()); 
}
