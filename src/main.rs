extern crate rand;
use rand::Rng;
use std::collections::{HashMap};

#[derive(Copy, Clone)]
struct Agent
{
    position: [i32;2],
    id: i32,
}

impl Agent
{
    pub fn step(&mut self, grid:& mut HashMap<(i32,i32), i32>)
    {
        let options: [[i32;2];4] = [[0,1],[0,-1],[-1,0],[1,0]];
        let delta: [i32;2] = options[rand::thread_rng().gen_range(0..4)];
        //todo: there is an issue here, probably in the adding code, where no new positions are chosen
        //println!("Agent {} started at {},{}",self.id,self.position[0],self.position[1]);

        self.position = [self.position[0] + delta[0], self.position[1] + delta[1]];

        let count = grid.entry((self.position[0],self.position[1])).or_insert(0);
        //println!("Agent {} moved to {},{}",self.id,self.position[0],self.position[1]);
        *count += 1;
    }
}

fn main() {
    println!("Hello, world!");
    const NUMBER_AGENTS:usize = 50;
    const STEPS:usize = 50;
    let mut agents:[Agent;NUMBER_AGENTS] = [Agent{position:[0,0], id: 0};NUMBER_AGENTS];
    let mut grid  = HashMap::new();
    grid.insert((0,0),NUMBER_AGENTS as i32);

    for i in 0..NUMBER_AGENTS
    {
        agents[i] = Agent{position:[0,0],id: i as i32};
    }

    for i in 0..STEPS
    {
        for agent in agents.iter_mut()
        {
            agent.step(&mut grid);
        }
        println!("Step {} completed", i);
    }

    for (key,val) in grid.iter()
    {
        println!("({},{}) was visited {} times", key.0,key.1, val);
    }


}
