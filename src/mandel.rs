use crate::game::map::Map;
use crate::packets::play::block::Block;
use std::time::Instant;

const BLOCKS: [Block; 9] = [Block::Lava, Block::RedConcrete, Block::Grass, Block::SlimeBlock, Block::Water, Block::HoneyBlock, Block::Dirt, Block::Bedrock, Block::BlackConcrete];

pub async fn compute(x1: f64, y1: f64, map: &mut Map) {
    let width = 1000; // we are going to draw a square of 1000 x 1000
    let max_iter = 30;
    let zoom = 250.;
    let now = Instant::now();

    for y in -width..width {
        for x in -width..width {
            let c_r = x as f64 / zoom + x1;
            let c_i = y as f64 / zoom + y1;
            let mut z_r = 0.0;
            let mut z_i = 0.0;
            let mut i = 0;

            while (z_r * z_r + z_i * z_i < 4.0) && i < max_iter {
                let tmp = z_r;
                z_r = z_r * z_r - z_i * z_i + c_r;
                z_i = 2.0 * z_i * tmp + c_i;
                i += 1;
            }

            if i == max_iter {
                map.set_block(x, 4, y, Block::WhiteConcrete).await;
            } else {
                let idx = i / BLOCKS.len();
                map.set_block(x, 4, y, BLOCKS[idx]).await;
            }
        }
    }

    let duration = now.elapsed();
    println!(
        "{:.5} seconds for whatever you did.",
        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
    );
}
