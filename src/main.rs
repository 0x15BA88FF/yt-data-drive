mod encoder;

fn main() {
    const RESOLUTIONS: ((u16, u16), (u16, u16), (u16, u16)) = (
        (3840, 2160),
        (1920, 1080),
        (1280, 720), 
    );

    encoder::run("test.txt", RESOLUTIONS.1)
}
