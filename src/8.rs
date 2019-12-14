// --- Day 8: Space Image Format ---
//
// https://adventofcode.com/2019/day/8
//
//
use console::style;

type Bytes = Vec<u8>;

#[derive(Debug, PartialEq)]
struct Layer {
    data: Bytes,
}

impl Layer {
    fn new(data: Bytes) -> Layer {
        Layer { data }
    }

    fn count(&self, n: u8) -> usize {
        let mut c = 0;
        for x in &self.data {
            if *x == n {
                c += 1;
            }
        }
        c
    }

    fn draw(&self, w: usize) {
        for chunk in self.data.chunks(w) {
            for c in chunk {
                if *c == 0 {
                    print!("{}", style(c).black());
                } else {
                    print!("{}", style(c).white());
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct SpaceImage {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}
impl SpaceImage {
    fn new(data: Bytes, w: usize, h: usize) -> SpaceImage {
        let wsz = w * h;
        let mut layers = vec![];
        for chunk in data.chunks(wsz) {
            layers.push(Layer::new(chunk.iter().copied().collect()))
        }

        SpaceImage {
            layers,
            width: w,
            height: h,
        }
    }

    fn layers(&mut self) -> &mut Vec<Layer> {
        &mut self.layers
    }

    fn decode_image(&mut self) -> Layer {
        let mut image = vec![];
        for idx in 0..(self.width * self.height) {
            let pixels = self.layers.iter().map(|x| x.data[idx]).collect::<Vec<u8>>();
            image.push(SpaceImage::decode_pixel(pixels));
        }
        Layer::new(image)
    }

    fn decode_pixel(pixels: Vec<u8>) -> u8 {
        let mut end = pixels.len() - 1;
        let mut col = pixels[end];
        loop {
            if pixels[end] != 2 {
                col = pixels[end];
            }
            if end == 0 {
                break;
            }
            end -= 1;
        }
        col
    }
}

fn main() {
    let raw_image_data = include_str!("input8.txt")
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let mut image = SpaceImage::new(raw_image_data, 25, 6);
    let layer = image
        .layers()
        .iter()
        .min_by(|x, y| x.count(0).cmp(&y.count(0)))
        .unwrap();
    println!("Part one: {}", layer.count(1) * layer.count(2));
    println!("Part two");
    image.decode_image().draw(25);
}

#[test]
fn test_sif_layers() {
    let mut image = SpaceImage::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 2, 0, 1, 2], 3, 2);
    image.layers.sort_by_key(|l| l.count(0));
    for layer in image.layers {
        if layer.count(0) != 0 {
            assert_eq!(2, layer.count(1) * layer.count(2));
        }
    }
}

#[test]
fn test_image_decoding() {
    let mut image = SpaceImage::new(vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0], 2, 2);
    let layer = image.decode_image();
    assert_eq!(Layer::new(vec![0, 1, 1, 0]), layer);
}
