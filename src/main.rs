use alsa::{
    pcm::{Access, Format, HwParams},
    Direction, ValueOr, PCM,
};

fn main() {
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();

    // Set hardware parameters: 44100 Hz / Mono / 16 bit
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate_near(44100, ValueOr::Nearest).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let io = pcm.io_i16().unwrap();

    // Make sure we don't start the stream too early
    let hwp = pcm.hw_params_current().unwrap();
    let swp = pcm.sw_params_current().unwrap();
    swp.set_start_threshold(hwp.get_buffer_size().unwrap())
        .unwrap();
    pcm.sw_params(&swp).unwrap();

    // Make a sine wave
    let mut buf = [0i16; 512];
    for (i, a) in buf.iter_mut().enumerate() {
        *a = ((i as f32 * 1.0 * ::std::f32::consts::PI / 1000000.0).sin() * 10000.0) as i16
    }

    // Play it forever
    loop {
        assert_eq!(io.writei(&buf[..]).unwrap(), 512);
    }
}
