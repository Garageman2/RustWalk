extern crate rand;
extern crate image;
use rand::Rng;
use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use image::{ImageBuffer, RgbImage, Rgb};

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

        self.position = [self.position[0] + delta[0], self.position[1] + delta[1]];

        let count = grid.entry((self.position[0],self.position[1])).or_insert(0);
        *count += 1;
    }
}

fn write_out(item: &HashMap<(i32,i32),i32>) ->std::io::Result<()>
{
    let mut file = File::create("Output.txt")?;
    for (key,val) in item.iter()
    {
        file.write_all(format!("({},{}) was visited {} times \n", key.0,key.1, val).as_bytes())?;
    }
    return Ok(());
}

fn main() {
    println!("Hello, world!");
    const NUMBER_AGENTS:usize = 100;
    const STEPS:usize = 100;
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

    write_out(&grid).ok();

    const SIZE: u32 = 256;

    let mut img: RgbImage = ImageBuffer::new(SIZE,SIZE);

    for x in 0..SIZE
    {
        for y in 0..SIZE
        {
            println!("The coordinates are {},{}", x,y);
            let mut result: u8;
            let a: i32 = (x as i32 - (SIZE/2) as i32);
            let b: i32 = (y as i32 - (SIZE/2) as i32);
            if grid.contains_key(&(a,b))
            {
                result = grid[&(a,b)].clamp(0,255) as u8;
            }
            else
            {
                 result = 0;
            }
            *img.get_pixel_mut(x,y) = Rgb([result,result,40]);
        }
    }
    img.save("Test.jpg").unwrap();

}
