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

#[test]
pub fn node_link_other(){
    let mut node = Node::new(46);
    node.next = Some(Box::new(Node::new(65)));
    node.insert_node(&mut Node::new(67));
    assert_eq!(String::from("Node value : 46, next value:  67"), node.to_string());
}
