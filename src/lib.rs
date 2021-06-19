use wasm_bindgen::prelude::*;
use::std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

const AUTO_SUCCESS: i8 = i8::MAX;
const AUTO_FAIL: i8 = i8::MIN;

#[wasm_bindgen]
#[derive(PartialEq,Eq,Hash,Copy,Clone)]
pub enum Token {
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

#[wasm_bindgen]
pub struct ChaosBag {
    token_counts: HashMap<Token, i8>,
    token_values: HashMap<Token, i8>,
    draw_again: HashMap<Token, bool>
}

#[derive(PartialEq,Eq,Hash)]
struct DrawAgainState {
    bless_count: i8,
    curse_count: i8,
    skull_count: i8,
    cultist_count: i8,
    tablet_count: i8,
    elder_thing_count: i8,
    //probability: f64
}

#[derive(PartialEq,Eq,Hash)]
pub struct FinalState {
    bless_count: i8,
    curse_count: i8,
    skull_count: i8,
    cultist_count: i8,
    tablet_count: i8,
    elder_thing_count: i8,
    final_draw: Token,
    //probability: f64
}

#[wasm_bindgen]
pub fn build_chaos_bag() -> ChaosBag {
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

#[wasm_bindgen]
impl ChaosBag {
	#[wasm_bindgen]
	pub fn set_token_count(&mut self, token: Token, count: i8) {
		if count != 0 {
			self.token_counts.insert(token, count);
		}
	}

	#[wasm_bindgen]
	pub fn set_token_value(&mut self, token: Token, value: i8) {
		self.token_values.insert(token, value);
	}

	#[wasm_bindgen]
	pub fn set_draw_again(&mut self, token: Token, draw_again: bool) {
		self.draw_again.insert(token, draw_again);
	}
}

fn get_final_states_str(states: &HashMap<FinalState, f64>) -> String {
    let mut s = String::new();
    writeln!(&mut s, "*------------------------------------------------------*").expect("Error formatting string");

    for (state, prob) in states.iter() {
        write!(&mut s, "| DA: ").expect("Error formatting string");
        if state.bless_count > 0 {
            write!(&mut s, "{}:{}", Token::Bless, state.bless_count).expect("Error formatting string");
        }
        if state.curse_count > 0 {
            write!(&mut s,"{}:{}", Token::Curse, state.curse_count).expect("Error formatting string");
        }
        if state.skull_count > 0 {
            write!(&mut s,"{}:{}", Token::Skull, state.skull_count).expect("Error formatting string");
        }
        if state.cultist_count > 0 {
            write!(&mut s,"{}:{}", Token::Cultist, state.cultist_count).expect("Error formatting string");
        }
        if state.tablet_count > 0 {
            write!(&mut s, "{}:{}", Token::Tablet, state.tablet_count).expect("Error formatting string");
        }
        if state.elder_thing_count > 0 {
            write!(&mut s,"{}:{}", Token::ElderThing, state.elder_thing_count).expect("Error formatting string");
        }
        writeln!(&mut s, "  Final Token: {}, P: {1:.4}", state.final_draw, prob).expect("Error formatting string");
    }

    writeln!(&mut s, "*------------------------------------------------------*").expect("Error formatting string");

    return s;
}

fn count_tokens(chaos_bag: &ChaosBag) -> i8 {
    let mut sum: i8 = 0;
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

fn draw_iter(chaos_bag: &ChaosBag, states: &HashMap<DrawAgainState, f64>, finished_states: &mut HashMap<FinalState, f64>, tokens_left: i8) -> HashMap<DrawAgainState, f64> {
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

fn compute_test_score(chaos_bag: &ChaosBag, final_state: &FinalState) -> i8 {
    let mut final_val: i8;

    if final_state.final_draw == Token::AutoFail {
        // return minimum if we drew auto fail
        return AUTO_FAIL;
    } else if final_state.final_draw == Token::ElderSign && chaos_bag.token_values[&Token::ElderSign] == AUTO_SUCCESS {
        // return max if we drew auto success elder sign
        return AUTO_SUCCESS;
    } else {
        println!("{}", final_state.final_draw);
        final_val = chaos_bag.token_values[&final_state.final_draw];
    }

    if chaos_bag.draw_again.contains_key(&Token::Bless) {
        final_val += chaos_bag.token_values[&Token::Bless] * final_state.bless_count;
    }
    if chaos_bag.draw_again.contains_key(&Token::Curse) {
        final_val += chaos_bag.token_values[&Token::Curse] * final_state.curse_count;
    }
    if chaos_bag.draw_again.contains_key(&Token::Skull) {
        final_val += chaos_bag.token_values[&Token::Skull] * final_state.skull_count;
    }
    if chaos_bag.draw_again.contains_key(&Token::Cultist) {
        final_val += chaos_bag.token_values[&Token::Cultist] * final_state.cultist_count;
    }
    if chaos_bag.draw_again.contains_key(&Token::Tablet) {
        final_val += chaos_bag.token_values[&Token::Tablet] * final_state.tablet_count;
    }
    if chaos_bag.draw_again.contains_key(&Token::ElderThing) {
        final_val += chaos_bag.token_values[&Token::ElderThing] * final_state.elder_thing_count;
    }
    
    return final_val;
}

fn compute_test_probabilities(chaos_bag: &ChaosBag, finished_states: &HashMap<FinalState,f64>) -> Vec<(i8,f64)> {
    let mut probabilities: Vec<(i8,f64)> = Vec::new();

    for (final_state, prob) in finished_states {
        let score = compute_test_score(chaos_bag, final_state);

        probabilities.push((score, *prob));
    }

    probabilities.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Compress final results combining equal scores
    let mut curr: usize = 0;

    while curr < probabilities.len() - 1 {
        let next = curr + 1;
        if probabilities[next].0 == probabilities[curr].0 {
            probabilities[curr].1 += probabilities.remove(next).1;
        } else if probabilities[next].0 > probabilities[curr].0 + 1 && probabilities[curr].0 != AUTO_FAIL && probabilities[next].0 != AUTO_SUCCESS {
            probabilities.insert(next, (probabilities[curr].0 + 1, 0.0));
        } else {
            curr += 1;
        }
    }

    return probabilities;
}

#[wasm_bindgen]
pub fn draw_bag(chaos_bag: &mut ChaosBag) -> JsValue {
    let mut initial_states: HashMap<DrawAgainState,f64> = HashMap::new();
    let mut finished_states: HashMap<FinalState,f64> = HashMap::new();
    initial_states.insert(DrawAgainState { bless_count: 0, curse_count: 0, skull_count: 0, cultist_count: 0, tablet_count: 0, elder_thing_count: 0}, 1.0);

    let token_count = count_tokens(&chaos_bag);
    let mut states = initial_states;
    
    for i in 0..30 {
        let new_states = draw_iter(&chaos_bag, &states, &mut finished_states, token_count - i);
        //let all_probs = sum_probabilities(&finished_states);
        //println!("{0:.12}", all_probs);
        states = new_states;
    }

    let results = compute_test_probabilities(&chaos_bag, &finished_states);

    //let s: String = get_final_states_str(&finished_states);

    return JsValue::from_serde(&results).unwrap();
}

#[test]
pub fn draw() {
    let mut my_chaos_bag = build_chaos_bag();

    // Test Bag
    my_chaos_bag.set_token_count(Token::PlusOne, 1);
    my_chaos_bag.set_token_count(Token::Zero, 2);
    my_chaos_bag.set_token_count(Token::MinusOne, 3);
    my_chaos_bag.set_token_count(Token::MinusTwo, 2);
    my_chaos_bag.set_token_count(Token::MinusThree, 1);
    my_chaos_bag.set_token_count(Token::MinusFour, 1);
    my_chaos_bag.set_token_count(Token::Skull, 2);
    my_chaos_bag.set_token_count(Token::Cultist, 1);
    my_chaos_bag.set_token_count(Token::AutoFail, 1);
    my_chaos_bag.set_token_count(Token::ElderSign, 1);
    //set_token_count(&mut my_chaos_bag, Token::Bless, 10);
    //set_token_count(&mut my_chaos_bag, Token::Curse, 10);

    my_chaos_bag.set_token_value(Token::Skull, -1);
    my_chaos_bag.set_token_value(Token::Cultist, -2);
    my_chaos_bag.set_token_value(Token::ElderSign, 2);  

    // //print_chaos_bag(&my_chaos_bag);

    // let mut initial_states: HashMap<DrawAgainState,f64> = HashMap::new();
    // let mut finished_states: HashMap<FinalState,f64> = HashMap::new();
    // initial_states.insert(DrawAgainState { bless_count: 0, curse_count: 0, skull_count: 0, cultist_count: 0, tablet_count: 0, elder_thing_count: 0}, 1.0);

    // let token_count = count_tokens(&my_chaos_bag);

    // //println!("{}", token_count);

    // let mut states = initial_states;
    

    // for i in 0..1 {
    //     let new_states = draw_iter(&my_chaos_bag, &states, &mut finished_states, token_count - i);
    //     let all_probs = sum_probabilities(&finished_states);
    //     println!("{0:.12}", all_probs);    
    //     states = new_states;
    // }

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

    draw_bag(&mut my_chaos_bag);
    
    // let s: String = get_final_states_str(&finished_states);
    // alert(&s);
}