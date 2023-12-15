use sky::readfile;

type Number = u32;

fn hash(s: &str) -> Number {
    let mut answer = 0;
    for c in s.chars() {
        let code = c as u32;
        answer += code;
        answer *= 17;
        answer %= 256;
    }
    answer
}

pub fn a() {
    let ctxt = readfile("15");
    let all = ctxt.value();
    let mut total = 0;
    for step in all.split(',') {
        total += hash(step);
    }
    println!("Sum of the hashes is: {total}");
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Lens {
    label: String,
    focal: Number,
}

impl Lens {
    fn new(label: &str, focal: &str) -> Self {
        let label = String::from(label);
        let focal: Number = focal
            .parse()
            .expect("Focal length {focal} should be an integer");
        Self { label, focal }
    }
}

#[derive(Clone, Debug)]
struct Array {
    boxes: [Vec<Lens>; 256],
}

impl Array {
    fn new() -> Self {
        let boxes = [(); 256].map(|_| Vec::new());
        Self { boxes }
    }

    fn remove(&mut self, bx: Number, label: &str) {
        self.boxes[bx as usize].retain(|lens| lens.label != label);
    }

    fn add_or_replace(&mut self, bx: Number, label: &str, focal: &str) {
        let new = Lens::new(label, focal);
        for lens in self.boxes[bx as usize].iter_mut() {
            if lens.label == label {
                *lens = new;
                return;
            }
        }
        self.boxes[bx as usize].push(new);
    }

    fn power(&self) -> Number {
        let mut total = 0;
        for (k, bx) in self.boxes.iter().enumerate() {
            for (n, lens) in bx.iter().enumerate() {
                let a = (k + 1) as Number;
                let b = (n + 1) as Number;
                total += a * b * lens.focal;
            }
        }
        total
    }
}

pub fn b() {
    let ctxt = readfile("15");
    let all = ctxt.value();
    let mut array = Array::new();
    for step in all.split(',') {
        if let Some(label) = step.strip_suffix('-') {
            let bx = hash(label);
            array.remove(bx, label);
        } else {
            let (label, focal) = step
                .split_once('=')
                .expect("The focal should be after an equals sign");
            let bx = hash(label);
            array.add_or_replace(bx, label, focal);
        }
    }
    let total = array.power();
    println!("Focusing power of the whole configuration is: {total}");
}
