//use std::io;

fn main() {
    let start = create_node(5);
}



struct node 
{
    data: u32,
    right:u32,
    left:u32,
}

fn create_node(node_data : u32) ->node
{
    node
    {
        data:node_data,
        right:0,
        left:0,
    }//because there is no semi colon the funcntion should return node
}

