use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;
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
    pub fn step(&mut self, grid:& mut HashMap<(i32,i32), i32>, rng:&mut ChaCha8Rng)
    {
        let options: [[i32;2];8] = [[0,1],[0,-1],[-1,0],[1,0],[1,1],[1,-1],[-1,1],[-1,-1]];
        let delta: [i32;2] = options[rng.gen_range(0..8)];

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
    const NUMBER_AGENTS:usize = 1000;
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
        let mut rng = ChaCha8Rng::from_entropy();
        for agent in agents.iter_mut()
        {
            agent.step(&mut grid, &mut rng);
            if i == STEPS -1
            {
                agent.dist();
            }
        }
    }

    println!("Walk Complete");

    write_out(&grid).ok();
    println!("Text output complete");

    const SIZE: u32 = 512;

    let mut img: RgbImage = ImageBuffer::new(SIZE,SIZE);

    for (key, value) in grid.into_iter()
    {
        let a = (key.0 + SIZE as i32/2).clamp(0, SIZE as i32) as u32;
        let b = (key.1 + SIZE as i32/2).clamp(0, SIZE as i32) as u32;
        *img.get_pixel_mut(a as u32,b as u32) = Rgb([0,value.clamp(0,255) as u8,0]);
    }

    img.save("Image.png").unwrap();
    println!("Image Output complete");

    let mut freq = HashMap::new();
    let mut max = 0;
    for agent in agents
    {
        let count: &mut i32 = freq.entry(agent.distance as i32).or_insert(0);
        *count += 1;
        if *count > max
        {
            max = *count;
        }
    }

    const GRAPH_SIZE: u32 = 256;
    const GRAPH_PAD: u32 = 2;

    let width: usize = (GRAPH_SIZE - (2 * GRAPH_PAD)) as usize/freq.len();

    let mut graph: RgbImage = ImageBuffer::from_fn(GRAPH_SIZE,GRAPH_SIZE,
 |x:u32,y:u32|{ return if (((x == GRAPH_SIZE-GRAPH_PAD) | (x == GRAPH_PAD-1)) &
    (GRAPH_PAD-1 <= y && y <= GRAPH_SIZE-GRAPH_PAD)) | (((y == GRAPH_PAD-1) |
    (y == GRAPH_SIZE-GRAPH_PAD)) & (GRAPH_PAD-1 <= x && x <= GRAPH_SIZE-GRAPH_PAD))
    { Rgb([40, 40, 40]) } else { Rgb([200, 200, 200]) } });

    let col = Rgb([0,20,80]);
    let mut offset: u32 = 0;
    for key in freq.keys().sorted()
    {
        let val: u32 = *freq.get(key).unwrap() as u32;
        let height: u32 = (((GRAPH_SIZE * 7 /8) / max as u32) * val) ;
        for x in offset..offset+width as u32+1
        {
            for y in GRAPH_SIZE-GRAPH_PAD-height..GRAPH_SIZE-GRAPH_PAD as u32
            {
                graph.put_pixel(x + 10, y, col);
            }
        }
        offset += width as u32;
    }

    graph.save("Graph.png").unwrap();
    println!("Graph complete");
}
