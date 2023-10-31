//MIT License

//Copyright (c) 2023 fbrega10

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.
//


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
//implementation of a Node (single pointer node data structure)
//It contains a value and the next value pointer (stored in the heap)
    pub fn new(val: i32) -> Self {
        Node {
            value: val,
            next: None,
        }
    }
    pub fn remove_next(&mut self) {
        match &self.next {
            None => println!("No next value"),
            Some(t) =>{
                match &t.next{
                None => self.next = None,
                Some(r) => self.next = Some(r.clone()),
                }
            },
        }
    }

    pub fn insert_node(&mut self,  other_node: &mut Node) {
        //method to insert a node between two, the pointer to the next one changes
        match &self.next{
            None => self.next = Some(Box::new(other_node.clone())),
            Some(_t) => {
                other_node.next = self.next.clone();
                self.next = Some(Box::new(other_node.clone()));
            },
        };
    }

    pub fn to_string(&self) -> String{
        match &self.next{
            Some(t) => format!("Node value : {}, next value:  {}", self.value, t.value),
            None => format!("Node value : {}", self.value),
        }
    }
}


