// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    let tile1 = Tile::Sand;
    let tile2 = Tile::Water(Pressure(190));
    let tile3 = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 200,
    });
    let tile4 = Tile::Brick(BrickStyle::Red);
    let tiles = vec![tile1, tile2, tile3, tile4];
    for tile in tiles {
        match tile {
            Tile::Brick(color @ BrickStyle::Gray | color @ BrickStyle::Red) => {
                println!("the brick color is {:?}", color)
            }
            Tile::Brick(other) => println!("{:?}", other),
            Tile::Water(pressure) if pressure.0 >= 10 => println!("high water pressure!"),
            Tile::Water(pressure) => println!("water pressure level {}", pressure.0),
            Tile::Grass | Tile::Dirt | Tile::Sand => println!("ground tile"),
            Tile::Treasure(TreasureChest {
                content: TreasureItem::Gold,
                amount,
            }) if amount >= 100 => println!("lots of gold"),
            _ => (),
        }
    }
}
