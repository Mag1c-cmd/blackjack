use std::{io};

fn clearscreen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[derive(Copy, Clone)]
enum Value {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    fn new(n: u8) -> Self {
        match n {
            2  => Value::Num(2),
            3  => Value::Num(3),
            4  => Value::Num(4),
            5  => Value::Num(5),
            6  => Value::Num(6),
            7  => Value::Num(7),
            8  => Value::Num(8),
            9  => Value::Num(9),
            10  => Value::Num(10),
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            14 => Value::Ace,
            _ => panic!(),
        }
    }

    fn show(&self) -> &str {
        match *self {
            Value::Num(2) => "2",
            Value::Num(3) => "3",
            Value::Num(4) => "4",
            Value::Num(5) => "5",
            Value::Num(6) => "6",
            Value::Num(7) => "7",
            Value::Num(8) => "8",
            Value::Num(9) => "9",
            Value::Num(10)=> "10",
            Value::Jack   => "J",
            Value::Queen  => "Q",
            Value::King   => "K",
            Value::Ace    => "A",
            _             => panic!(),
        }
    }

    fn value(&self) -> u8 {
        match *self {
            Value::Num(2) => 2,
            Value::Num(3) => 3,
            Value::Num(4) => 4,
            Value::Num(5) => 5,
            Value::Num(6) => 6,
            Value::Num(7) => 7,
            Value::Num(8) => 8,
            Value::Num(9) => 9,
            Value::Num(10) => 10,
            Value::Jack   => 10,
            Value::Queen  => 10,
            Value::King   => 10,
            Value::Ace    => 11,
            _             => panic!(),
        }
    }
}

#[derive(Copy, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn new(n: u8) -> Self {
        match n {
            0  => Suit::Clubs,
            1  => Suit::Diamonds,
            2  => Suit::Hearts,
            3  => Suit::Spades,
            _ => panic!(),
        }
    }

    fn value(&self) -> char {
        match *self {
            Suit::Clubs    => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts   => 'H',
            Suit::Spades   => 'S',
        }
    }
}

#[derive(Copy, Clone)]
struct Card {
    value: Value,
    suit: Suit,
    hidden: bool,
}

impl Card {
    fn new(value: Value, suit: Suit) -> Self {
        Self {
            value,
            suit,
            hidden: true,
        }
    }

    fn show(&self) {
        println!("Card value: {}, Card suit: {}", if !self.hidden { self.value.show() } else { "Hidden"}, if !self.hidden { self.suit.value() } else { '-' });
    }

    fn reveal(&mut self) {
        self.hidden = false;
    }
}

struct Deck {
    retreivable_cards: Vec<Card>,
    retreived_cards: Vec<Card>,
}

#[allow(dead_code)]
impl Deck {
    fn new() -> Self {
        let mut retreivable_cards: Vec<Card> = Vec::new();
        let retreived_cards: Vec<Card>   = Vec::new();
        for s in 0..=3 {
            for n in 2..=14 {
                let c: Card = Card::new(Value::new(n as u8), Suit::new(s as u8));
                retreivable_cards.push(c);
            }
        }

        Self {
            retreivable_cards,
            retreived_cards,
        }
    }

    fn shuffle(&mut self) {
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        let retreivable_cards: &mut Vec<Card> = &mut self.retreivable_cards;
        
        retreivable_cards.shuffle(&mut thread_rng());
    }

    fn show(&self) {
        for card in &self.retreivable_cards {
            card.show();
        }
    }

    fn retreive(&mut self, n: u8) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::new();
        for _ in 0..n {
            let card = self.retreivable_cards.first().unwrap();

            hand.append(&mut vec![*card]);
            self.retreived_cards.append(&mut vec![*card]);
            self.retreivable_cards.remove(0);
        }

        hand
    }
}

struct Hand {
    name: String,
    cards: Vec<Card>,
    value: u8,
}

impl Hand {
    fn new(name: String, cards: Vec<Card>) -> Self {
        let mut value: u8 = 0;
        for card in &cards {
            if !card.hidden {
                value += card.value.value();
            }
        }

        Self {
            name,
            cards,
            value,
        }
    }

    fn show(&self) {
        println!("-------------------------------");
        println!("{}", self.name);
        for card in &self.cards {
            card.show();
        }
        println!("Current hand value {}", &self.value);
        println!("-------------------------------");
    }

    fn recalulate_value(&mut self) {
        let mut value: u8 = 0;
        for card in &self.cards {
            if !card.hidden {
                value += card.value.value();
            }
        }

        self.value = value;
    }
}

struct Game {
    deck: Deck,
    dealer: Hand,
    players: Vec<Hand>,
}

impl Game {
    fn new(n_players: u8) -> Self {
        let mut deck: Deck = Deck::new();
        deck.shuffle();

        let dealer: Hand = Hand::new("Dealer".to_string(), deck.retreive(2));
        
        let mut players: Vec<Hand> = Vec::new();

        for i in 0..n_players as u8 {
            let name = format!("Player {}", i+1);
            let player = Hand::new(name, deck.retreive(2));
            players.append(&mut vec![player]);
        }
        
        Self {
            deck,
            dealer,
            players,
        }
    }
}

fn gameloop (mut game: Game) {
    let mut finished: bool = false;
    clearscreen();
    game.dealer.show();
    for player in &game.players {
        player.show();
    }
    println!("Press enter to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    while !finished {
        clearscreen();
        for card in &mut game.dealer.cards {
            card.reveal();
        }

        game.dealer.recalulate_value();

        // TODO: Check if dealer needs to hit

        game.dealer.show();
        
        for player in &mut game.players {
            for card in &mut player.cards {
                card.reveal();
            }
            player.recalulate_value();
            player.show();
        }

        // TODO: Compare hand values with each other.


        println!("Press enter to continue");

        // TODO: Give a set of options, hit, stand, etc. per player

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

pub fn new_game() {
    clearscreen();
    println!("How many players do you wants?");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_players: u8 = input.trim().parse().unwrap();

    let game = Game::new(n_players);
    gameloop(game);

}

