use macro_impl::{PiFn, SynChatty};

#[derive(PiFn, SynChatty)]
struct Hello {
    world: String,
    goodbye: u32,
}

fn main() {
    let h = Hello {
        world: "mars!".to_string(),
        goodbye: 13,
    };

    h.introspect();
    println!("world value: {}", h.world());
    println!("goodbye value: {}", h.goodbye());
}
