use audio_visualization::renderer::run;

fn main() {
    pollster::block_on(run());
}
