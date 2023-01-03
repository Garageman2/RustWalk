extern crate rand;
extern crate image;
extern crate itertools;
use rand::Rng;
use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use image::{ImageBuffer, RgbImage, Rgb};
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Agent
{
    position: [i32;2],
    id: u32,
    distance: f32,
}

impl Agent
{
    pub fn step(&mut self, grid:& mut HashMap<(i32,i32), i32>)
    {
        let options: [[i32;2];8] = [[0,1],[0,-1],[-1,0],[1,0],[1,1],[1,-1],[-1,1],[-1,-1]];
        let delta: [i32;2] = options[rand::thread_rng().gen_range(0..options.len())];

        self.position = [self.position[0] + delta[0], self.position[1] + delta[1]];

        let count = grid.entry((self.position[0],self.position[1])).or_insert(0);
        *count += 1;
    }

    fn new(position:[i32;2], id:u32) -> Agent
    {
        return Agent{position, id,distance: 0.0 };
    }

    fn dist(&mut self)
    {
        self.distance = ((self.position[0] as f32).powi(2) + ((self.position[1] as f32).powi(2))).sqrt();
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
    const NUMBER_AGENTS:usize = 500;
    const STEPS:usize = 3000;
    let mut agents:[Agent;NUMBER_AGENTS] = [Agent::new([0,0], 0);NUMBER_AGENTS];
    let mut grid  = HashMap::new();
    grid.insert((0,0),NUMBER_AGENTS as i32);

    for i in 0..NUMBER_AGENTS
    {
        agents[i].id = i as u32;
    }

    for i in 0..STEPS
    {
        for agent in agents.iter_mut()
        {
            agent.step(&mut grid);
            if i == STEPS -1
            {
                agent.dist();
            }
        }
    }

    println!("Walk Complete");

    write_out(&grid).ok();
    println!("Text output complete");

    const SIZE: u32 = 256;

    let mut img: RgbImage = ImageBuffer::new(SIZE,SIZE);

    for x in 0..SIZE
    {
        for y in 0..SIZE
        {
            //TODO: refactor to go by key value pair instead so that outer pixels arent iterated
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
            *img.get_pixel_mut(x,y) = Rgb([0,result,0]);
        }
    }
    img.save("Image.png").unwrap();
    println!("Image Output complete");

    let mut freq = HashMap::new();
    let mut max = 0;
    for agent in agents
    {
        let count: &mut i32 = freq.entry((agent.distance as i32)).or_insert(0);
        *count += 1;
        if *count > max
        {
            max = *count;
        }
    }
    //TODO: note that 126 and 2 are hardcoded from 128 x 128
    let mut width: usize = freq.len() / 124;

    let mut graph: RgbImage = ImageBuffer::from_fn(256,256,|x:u32,y:u32|{ return if (((x == 254) | (x == 1)) & (1 <= y && y <= 254)) | (((y == 1) | (y == 254)) & (1 <= x && x <= 254)) { Rgb([40, 40, 40]) } else { Rgb([200, 200, 200]) } });

    let col = Rgb([0,20,80]);
    let mut offset: u32 = 0;
    for key in freq.keys().sorted()
    {
        let val: u32 = *freq.get(key).unwrap() as u32;
        println!("val is {} and key is {}", val, key);
        let height: u32 = ((200 / max) * val as i32) as u32;
        /*for x in (offset..offset+width as u32)
        {*/
        for y in (253-height..253 as u32)
        {
            graph.put_pixel((*key as u32) + 2, y, col);
        }/*
        }
        offset += width as u32;
        println!("{} has val {}", key, freq.get(key).unwrap());*/
    }

    graph.save("Graph.png").unwrap();
    println!("Graph complete");
}
