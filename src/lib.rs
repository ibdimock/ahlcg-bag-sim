use wasm_bindgen::prelude::*;
use::std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

    
#[derive(PartialEq,Eq,Hash,Copy,Clone)]
enum Token {
    PlusOne,
    Zero,
    MinusOne,
    MinusTwo,
    MinusThree,
    MinusFour,
    MinusFive,
    MinusSix,
    MinusSeven,
    MinusEight,
    Skull,
    Cultist,
    ElderThing,
    Tablet,
    ElderSign,
    AutoFail,
    Bless,
    Curse
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::PlusOne      => write!(f, "[+1]"),
            Token::Zero         => write!(f, "[ 0]"),
            Token::MinusOne     => write!(f, "[-1]"),
            Token::MinusTwo     => write!(f, "[-2]"),
            Token::MinusThree   => write!(f, "[-3]"),
            Token::MinusFour    => write!(f, "[-4]"),
            Token::MinusFive    => write!(f, "[-5]"),
            Token::MinusSix     => write!(f, "[-6]"),
            Token::MinusSeven   => write!(f, "[-7]"),
            Token::MinusEight   => write!(f, "[-8]"),
            Token::Skull        => write!(f, "[Sk]"),
            Token::Cultist      => write!(f, "[Cl]"),
            Token::ElderThing   => write!(f, "[ET]"),
            Token::Tablet       => write!(f, "[Tb]"),
            Token::ElderSign    => write!(f, "[ES]"),
            Token::AutoFail     => write!(f, "[AF]"),
            Token::Bless        => write!(f, "[Bl]"),
            Token::Curse        => write!(f, "[Cr]"),
        }
    }
}

struct ChaosBag {
    token_counts: HashMap<Token, u8>,
    token_values: HashMap<Token, i8>,
    draw_again: HashMap<Token, bool>    
}

#[derive(PartialEq,Eq,Hash)]
struct DrawAgainState {
    bless_count: u8,
    curse_count: u8,
    skull_count: u8,
    cultist_count: u8,
    tablet_count: u8,
    elder_thing_count: u8,
    //probability: f64
}

#[derive(PartialEq,Eq,Hash)]
struct FinalState {
    bless_count: u8,
    curse_count: u8,
    skull_count: u8,
    cultist_count: u8,
    tablet_count: u8,
    elder_thing_count: u8,
    final_draw: Token,
    //probability: f64
}

fn build_chaos_bag() -> ChaosBag {
    let mut chaos_bag = ChaosBag {
        token_counts: HashMap::new(),
        token_values: HashMap::new(),
        draw_again: HashMap::new()
    };

    // Set up known values
    chaos_bag.token_values.insert(Token::PlusOne, 1);
    chaos_bag.token_values.insert(Token::Zero, 0);
    chaos_bag.token_values.insert(Token::MinusOne, -1);
    chaos_bag.token_values.insert(Token::MinusTwo, -2);
    chaos_bag.token_values.insert(Token::MinusThree, -3);
    chaos_bag.token_values.insert(Token::MinusFour, -4);
    chaos_bag.token_values.insert(Token::MinusFive, -5);
    chaos_bag.token_values.insert(Token::MinusSix, -6);
    chaos_bag.token_values.insert(Token::MinusSeven, -7);
    chaos_bag.token_values.insert(Token::MinusEight, -8);
    chaos_bag.token_values.insert(Token::AutoFail, -128);
    chaos_bag.token_values.insert(Token::Bless, 2);
    chaos_bag.token_values.insert(Token::Curse, -2);

    // Set up default draw again statuses for Bless/Curse
    chaos_bag.draw_again.insert(Token::Bless, true);
    chaos_bag.draw_again.insert(Token::Curse, true);

    return chaos_bag;
}

fn set_token_count(chaos_bag: &mut ChaosBag, token: Token, count: u8) {
    chaos_bag.token_counts.insert(token, count);
}

fn set_token_value(chaos_bag: &mut ChaosBag, token: Token, value: i8) {
    chaos_bag.token_values.insert(token, value);
}

fn set_draw_again(chaos_bag: &mut ChaosBag, token: Token, draw_again: bool) {
    chaos_bag.draw_again.insert(token, draw_again);
}

fn print_chaos_bag(chaos_bag: &ChaosBag) {
    println!("*------------------------------------------------------*");

    for token in chaos_bag.token_counts.keys() {
        println!("| {} - count:{:3}  |  value:{:4}  |  draw_again:{} |", token, chaos_bag.token_counts[token], chaos_bag.token_values[token], chaos_bag.draw_again.contains_key(token))
    }

    println!("*------------------------------------------------------*");
}

fn get_final_states_str(states: &HashMap<FinalState, f64>) -> String {
    let mut s = String::new();
    writeln!(&mut s, "*------------------------------------------------------*");

    for (state, prob) in states.iter() {
        write!(&mut s, "| DA: ");
        if state.bless_count > 0 {
            write!(&mut s, "{}:{}", Token::Bless, state.bless_count);
        }
        if state.curse_count > 0 {
            write!(&mut s,"{}:{}", Token::Curse, state.curse_count);
        }
        if state.skull_count > 0 {
            write!(&mut s,"{}:{}", Token::Skull, state.skull_count);
        }
        if state.cultist_count > 0 {
            write!(&mut s,"{}:{}", Token::Cultist, state.cultist_count);
        }
        if state.tablet_count > 0 {
            write!(&mut s, "{}:{}", Token::Tablet, state.tablet_count);
        }
        if state.elder_thing_count > 0 {
            write!(&mut s,"{}:{}", Token::ElderThing, state.elder_thing_count);
        }
        writeln!(&mut s, "  Final Token: {}, P: {1:.4}", state.final_draw, prob);
    }

    writeln!(&mut s, "*------------------------------------------------------*");

    return s;
}

fn count_tokens(chaos_bag: &ChaosBag) -> u8 {
    let mut sum: u8 = 0;
    for count in chaos_bag.token_counts.values() {
        sum += *count;
    }
    return sum;
}

fn sum_probabilities(states: &HashMap<FinalState, f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for prob in states.values() {
        sum += *prob;
    }
    return sum;
}

fn new_final_state(da_state: &DrawAgainState, token: Token) -> FinalState {
    FinalState {
        bless_count:        da_state.bless_count,
        curse_count:        da_state.curse_count,
        skull_count:        da_state.skull_count,
        cultist_count:      da_state.cultist_count,
        tablet_count:       da_state.tablet_count,
        elder_thing_count:  da_state.elder_thing_count,
        final_draw:         token
    }
}

fn new_draw_again_state(da_state: &DrawAgainState, token: Token) -> DrawAgainState {
    match token {
        Token::Bless => {
            DrawAgainState {
                bless_count:        da_state.bless_count + 1,
                curse_count:        da_state.curse_count,
                skull_count:        da_state.skull_count,
                cultist_count:      da_state.cultist_count,
                tablet_count:       da_state.tablet_count,
                elder_thing_count:  da_state.elder_thing_count
            }
        },
        Token::Curse => {
            DrawAgainState {
                bless_count:        da_state.bless_count,
                curse_count:        da_state.curse_count + 1,
                skull_count:        da_state.skull_count,
                cultist_count:      da_state.cultist_count,
                tablet_count:       da_state.tablet_count,
                elder_thing_count:  da_state.elder_thing_count
            }
        },
        Token::Skull => {
            DrawAgainState {
                bless_count:        da_state.bless_count,
                curse_count:        da_state.curse_count,
                skull_count:        da_state.skull_count + 1,
                cultist_count:      da_state.cultist_count,
                tablet_count:       da_state.tablet_count,
                elder_thing_count:  da_state.elder_thing_count
            }
        },
        Token::Cultist => {
            DrawAgainState {
                bless_count:        da_state.bless_count,
                curse_count:        da_state.curse_count,
                skull_count:        da_state.skull_count,
                cultist_count:      da_state.cultist_count + 1,
                tablet_count:       da_state.tablet_count,
                elder_thing_count:  da_state.elder_thing_count
            }
        },
        Token::Tablet => {
            DrawAgainState {
                bless_count:        da_state.bless_count,
                curse_count:        da_state.curse_count,
                skull_count:        da_state.skull_count,
                cultist_count:      da_state.cultist_count,
                tablet_count:       da_state.tablet_count + 1,
                elder_thing_count:  da_state.elder_thing_count
            }
        },
        Token::ElderThing => {
            DrawAgainState {
                bless_count:        da_state.bless_count,
                curse_count:        da_state.curse_count,
                skull_count:        da_state.skull_count,
                cultist_count:      da_state.cultist_count,
                tablet_count:       da_state.tablet_count,
                elder_thing_count:  da_state.elder_thing_count + 1
            }
        },
        _ => {
            DrawAgainState {
                bless_count:        da_state.bless_count,
                curse_count:        da_state.curse_count,
                skull_count:        da_state.skull_count,
                cultist_count:      da_state.cultist_count,
                tablet_count:       da_state.tablet_count,
                elder_thing_count:  da_state.elder_thing_count
            }
        }
    }
    
}

fn draw_iter(chaos_bag: &ChaosBag, states: &HashMap<DrawAgainState, f64>, finished_states: &mut HashMap<FinalState, f64>, tokens_left: u8) -> HashMap<DrawAgainState, f64> {
    let mut new_states: HashMap<DrawAgainState, f64> = HashMap::new();
    let this_draw_p: f64 = 1.0 / (tokens_left as f64);

    for (state, prob) in states.iter() {
        for token in chaos_bag.token_counts.keys() {
            if !chaos_bag.draw_again.contains_key(token) {
                let count = chaos_bag.token_counts[token] as f64;
                // If this token does not cause another draw
                let new_state: FinalState = new_final_state(state, *token);
                if finished_states.contains_key(&new_state) {
                    *finished_states.get_mut(&new_state).unwrap() += count * prob * this_draw_p;
                } else {
                    finished_states.insert(new_state, count * prob * this_draw_p);
                }
            } else {
                // If this token does cause another draw

                // Make sure this state has not already drawn all of this
                if  *token == Token::Bless && state.bless_count >= chaos_bag.token_counts[&Token::Bless] ||
                    *token == Token::Curse && state.curse_count >= chaos_bag.token_counts[&Token::Curse] ||
                    *token == Token::Skull && state.skull_count >= chaos_bag.token_counts[&Token::Skull] ||
                    *token == Token::Cultist && state.cultist_count >= chaos_bag.token_counts[&Token::Cultist] ||
                    *token == Token::Tablet && state.tablet_count >= chaos_bag.token_counts[&Token::Tablet] ||
                    *token == Token::ElderThing && state.elder_thing_count >= chaos_bag.token_counts[&Token::ElderThing]
                {
                    // Cannot draw this token again as it has already been draw as many times it is in the bag
                    continue;
                } else {
                    let count: f64 = match token {
                        Token::Bless => { (chaos_bag.token_counts[&Token::Bless] - state.bless_count) as f64 },
                        Token::Curse => { (chaos_bag.token_counts[&Token::Curse] - state.curse_count) as f64 },
                        Token::Skull => { (chaos_bag.token_counts[&Token::Skull] - state.skull_count) as f64 },
                        Token::Cultist => { (chaos_bag.token_counts[&Token::Cultist] - state.cultist_count) as f64 },
                        Token::Tablet => { (chaos_bag.token_counts[&Token::Tablet] - state.tablet_count) as f64 },
                        Token::ElderThing => { (chaos_bag.token_counts[&Token::ElderThing] - state.elder_thing_count) as f64 },
                        _ => chaos_bag.token_counts[token] as f64
                    };

                    let new_state: DrawAgainState = new_draw_again_state(state, *token);
                    if new_states.contains_key(&new_state) {
                        *new_states.get_mut(&new_state).unwrap() += count * prob * this_draw_p;
                    } else {
                        new_states.insert(new_state, count * prob * this_draw_p);
                    }
                }
            }
        }
    }

    return new_states;
}

fn main() {
    draw();
}


#[wasm_bindgen]
pub fn draw() {
    let mut my_chaos_bag = build_chaos_bag();

    // Test Bag
    set_token_count(&mut my_chaos_bag, Token::PlusOne, 1);
    set_token_count(&mut my_chaos_bag, Token::Zero, 2);
    set_token_count(&mut my_chaos_bag, Token::MinusOne, 3);
    set_token_count(&mut my_chaos_bag, Token::MinusTwo, 2);
    set_token_count(&mut my_chaos_bag, Token::MinusThree, 1);
    set_token_count(&mut my_chaos_bag, Token::MinusFour, 1);
    set_token_count(&mut my_chaos_bag, Token::Skull, 2);
    set_token_count(&mut my_chaos_bag, Token::Cultist, 1);
    set_token_count(&mut my_chaos_bag, Token::AutoFail, 1);
    set_token_count(&mut my_chaos_bag, Token::ElderSign, 1);
    //set_token_count(&mut my_chaos_bag, Token::Bless, 10);
    //set_token_count(&mut my_chaos_bag, Token::Curse, 10);

    set_token_value(&mut my_chaos_bag, Token::Skull, -1);
    set_token_value(&mut my_chaos_bag, Token::Cultist, -2);
    set_token_value(&mut my_chaos_bag, Token::ElderSign, 2);  

    //print_chaos_bag(&my_chaos_bag);

    let mut initial_states: HashMap<DrawAgainState,f64> = HashMap::new();
    let mut finished_states: HashMap<FinalState,f64> = HashMap::new();
    initial_states.insert(DrawAgainState { bless_count: 0, curse_count: 0, skull_count: 0, cultist_count: 0, tablet_count: 0, elder_thing_count: 0}, 1.0);

    let token_count = count_tokens(&my_chaos_bag);

    //println!("{}", token_count);

    let mut states = initial_states;
    

    for i in 0..1 {
        let new_states = draw_iter(&my_chaos_bag, &states, &mut finished_states, token_count - i);
        let all_probs = sum_probabilities(&finished_states);
        println!("{0:.12}", all_probs);    
        states = new_states;
    }

    // let new_states = draw_iter(&my_chaos_bag, &initial_states, &mut finished_states, token_count);
    // let all_probs = sum_probabilities(&finished_states);
    // println!("{0:.4}", all_probs);

    // let new_states2 = draw_iter(&my_chaos_bag, &new_states, &mut finished_states, token_count - 1);
    // let all_probs = sum_probabilities(&finished_states);
    // println!("{0:.4}", all_probs);

    // let _ = draw_iter(&my_chaos_bag, &new_states2, &mut finished_states, token_count - 2);
    // let all_probs = sum_probabilities(&finished_states);
    // println!("{0:.4}", all_probs);

    //print_final_states(&finished_states);
    
    let s: String = get_final_states_str(&finished_states);
    alert(&s);
}