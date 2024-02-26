use rand::seq::SliceRandom;

fn main() {
    let words = vec!["chipi", "chapa", "dubi", "daba"];
    let correct = vec![0, 1, 2, 3];

    let mut rng = rand::thread_rng();

    loop {
        let mut this = vec![0; 4];
        this[..4].clone_from_slice(&correct);
        this.shuffle(&mut rng);

        for &t in &this {
            print!("{} {} ", words[t], words[t]);
        }

        let mut pairs = this.iter().zip(correct.iter());
        if pairs.all(|(x, y)| x == y) {
            break;
        }
    }
    print!("mágico mi dubi dubi boom boom boom boom\n");
}
