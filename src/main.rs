use std::env;
use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};

struct MyWindowHandler{
    drawWidth: u32,
    drawHeight: u32,
    numLayers: u32,
    heap: Vec<u32>,
    heapText: Vec<FormattedTextBlock>
}
impl WindowHandler for MyWindowHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D){
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        
        for i in 0..self.heap.len(){
            let layer: u32 = (i+1).ilog2();
            let layerIndex: u32 = i as u32 % 2_u32.pow(layer);
            let divide: u32 = self.drawWidth / 2_u32.pow(layer + 1);
            let p = (
                (divide*(layerIndex*2 + 1)) as f32,
                (layer * self.drawHeight) as f32/ (self.numLayers) as f32 + 120_f32
            );
            graphics.draw_circle((p.0 + 12.5, p.1 + 12.5), 25.0, Color::WHITE);
            graphics.draw_text(p, Color::BLACK, &self.heapText[i]);
        }
    }
}
pub fn main() -> Result<(), std::io::Error> {
    let heap: Vec<u32> = env::args().skip(1).map(|s| s.parse().expect("numbers please")).collect();
    //check validity of binary heap
    for i in 1..heap.len(){
        if heap[i] > heap[(i - 1) / 2]{
            panic!("Invalid heap");
        }
    };
    let numLayers: u32 = heap.len().ilog2();
    let drawWidth: u32 = 150 * 2_u32.pow(numLayers);
    let drawHeight: u32 = 50 * numLayers;
    let font = Font::new(include_bytes!("./NotoSans-Regular.ttf")).unwrap();
    let heapText = env::args().skip(1).map(|s| font.layout_text(s.as_str(), 32.0, TextOptions::new())).collect();

    let window = Window::new_centered("test", (100 + drawWidth, drawHeight + 200)).unwrap();
    window.run_loop(MyWindowHandler{ drawWidth, drawHeight, numLayers, heap, heapText });
}
