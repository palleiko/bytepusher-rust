use std::io::Read;

const MEM_SIZE: usize           = 0x1_000_000;
const N_INSTRS: u32             = 65536;
const SCREEN_X: u32             = 0x100;
const SCREEN_Y: u32             = 0x100;
const KEYBOARD_STATE_OFF: usize = 0;

struct PixelColor {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}
/// Return a buffer of the program's memory and bytecode file
fn load_prog(filename: String) -> Result<Vec<u8>, std::io::Error> {

    // let mut buffer: Vec<u8>= vec![0; MEM_SIZE];
    let mut mem: Vec<u8> = Vec::new();

    let mut f: std::fs::File = std::fs::File::open(filename)?;
    f.read_to_end(&mut mem)?;

    mem.resize(MEM_SIZE, 0);

    Ok(mem)
}

fn render_frame(mut mem: Vec<u8>) {
    let mut ip = (mem[2] as usize) << 16 |
                 (mem[3] as usize) << 8 |
                 (mem[4] as usize);
    for _ in 0..N_INSTRS {
        println!("pre-IP: 0x{:x?}", ip);

        let oper1 = (mem[ip + 0] as usize) << 16 |
                    (mem[ip + 1] as usize) << 8 |
                    (mem[ip + 2] as usize);

        let oper2 = (mem[ip + 3] as usize) << 16 |
                    (mem[ip + 4] as usize) << 8 |
                    (mem[ip + 5] as usize);

        mem[oper2] = mem[oper1];
        println!("IP: 0x{:x?}", ip);
        ip = (mem[ip + 6] as usize) << 16 |
             (mem[ip + 7] as usize) << 8 |
             (mem[ip + 8] as usize);
    }

    for x in 0..SCREEN_X {
        for y in 0..SCREEN_Y {
            println!("{:02} * {:02}", x, y)
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <bytepusher file>", &args[0]);
        std::process::exit(2);
    }

    let filename = &args[1];
    let mut mem = load_prog(filename.to_string())?;

    render_frame(mem);

    Ok(())
}
