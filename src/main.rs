mod card;
mod hand;
mod deck;
mod step;


fn main() {
    println!("Hello, world!");
    let c = card::Card::new("Copper", "Treasure", 0);
    let s = card::Card::new("Silver", "Treasure", 0);
    let g = card::Card::new("Gold", "Treasure", 0);

    println!("Card: {:?}", c);
    println!("Card: {:?}", s);
    println!("Card: {:?}", g);

}
