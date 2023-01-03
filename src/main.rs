extern crate rand;
use rand::Rng;
use std::collections::{hash_map, HashMap};

#[derive(Copy, Clone)]
struct Agent
{
    position: [i32;2],
}

impl Agent
{
    pub fn step(&mut self, grid:& mut HashMap<(i32,i32), i32>)
    {
        let options: [[i32;2];4] = [[0,1],[0,-1],[-1,0],[1,0]];
        let delta: [i32;2] = options[rand::thread_rng().gen_range(0..4)];
        //println!("The move is {},{}", delta[0],delta[1]);
        //todo: there is an issue here, probably in the adding code, where no new positions are chosen
        self.position = [self.position[0] + delta[0], self.position[1] + delta[1]];
        let count = grid.entry((self.position[0],self.position[1])).or_insert(0);
        *count += 1;
    }
}

fn main() {
    println!("Hello, world!");
    //create a hash map and append found coordinates to it
    const NUMBER_AGENTS:usize = 50;
    const STEPS:usize = 10;
    let agents:[Agent;NUMBER_AGENTS] = [Agent{position:[0,0]};NUMBER_AGENTS];
    let mut grid  = HashMap::new();
    grid.insert((0,0),NUMBER_AGENTS as i32);

    let mut i: u32 = 0;
    while i < STEPS as u32
    {
        for mut agent in agents
        {
            agent.step(&mut grid);
        }
        i+=1;
    }

    for (key,val) in grid.iter()
    {
        println!("({},{}) was visited {} times", key.0,key.1, val);
    }


}
