use std::fmt::Debug;


pub trait StateTransition : Clone + Debug {
    const DEF : Self;
}
// #[derive(PartialEq)]
// pub enum SearchValue {f32}
pub trait SearchState<T : StateTransition> : Debug {
    // return the next state given this state and the transition
    fn state_transition(&self, t : &T) -> Self;

    // give a list of possible transitions from this state
    fn possible_transitions(&self) -> Vec<T>;

    // return a value which is a PESSIMISTIC estimate of all subsequent states. used as pruning heuristic
    fn no_better_than(&self) -> f32;



    // return a value which is a PESSIMISTIC estimate of all subsequent states. used as pruning heuristic
    fn no_worse_than(&self) -> f32;

    // return the value of THIS node. used to compare destinations
    fn value(&self) -> f32;
}

fn dank() {
    println!("OK");
}

//returns whether it exhausted its subtree
fn a_star<S,T> (t:&T, s:&S, depth_left:u32, lb:&mut f32, best: &mut T) -> bool
        where S : SearchState<T>, T : StateTransition {
    let h = s.no_worse_than();
    if h > *lb {
    println!("`{:?}` \t best: {} for {:?}", s, *lb, best);
        *lb = h;
        *best = t.clone();
    }
    let step_choices = s.possible_transitions();
    if depth_left > 0 {
        let mut completed = true;
        let mut nexts : Vec<S> =
            step_choices.iter().map(|choice| s.state_transition(choice)).collect();
        nexts.retain(|n| n.no_better_than() > *lb);
        nexts.sort_by(
            |n1, n2|
            n1.no_better_than().partial_cmp(& n2.no_better_than()).unwrap()
        );
        for next in nexts {
            if !a_star(t, &next, depth_left-1, lb, best ) {
                completed = false;
            }
        }
        completed
    } else {
        //only completed now if there was nothing left to do anyway
        step_choices.is_empty()
    }
}

pub fn solve<S,T> (state : &S) -> T
        where S : SearchState<T>, T : StateTransition {
    let GOOD_ENOUGH = 0.9;
    let mut lb = 0.0;
    let mut best = T::DEF;
    let choices = state.possible_transitions();
    let mut max_depth = 1;
    loop {
        for c in choices.iter() {
            let mut completed = true;
            if ! a_star(c, state, max_depth, &mut lb, &mut best) {
                completed = false;
            }
            if lb > GOOD_ENOUGH || completed {
                return best;
            }
        }
        max_depth += 1;
    }
}


#[cfg(test)]
mod tests {
    use {dank, solve};
    use {StateTransition, SearchState};

    impl StateTransition for char {
        const DEF: char = 'A';
    }

    #[derive(Debug)]
    struct Guess {
        s : String,
    }

    impl SearchState<char> for Guess {
        fn state_transition(&self, t : &char) -> Self {
            let mut x = self.s.clone();
            x.push(*t);
            Guess {
                s : x,
            }
        }

        fn possible_transitions(&self) -> Vec<char> {
            vec!['A', 'B', 'C', 'D']
        }

        fn no_worse_than(&self) -> f32 {
            let goal = "AAABBBCCC";
            let mut right = 0;
            for (g, z) in self.s.chars().zip(goal.chars()) {
                if g == z {
                    right += 1;
                }
            }
            right as f32 / (goal.len() as f32)
        }

        fn no_better_than(&self) -> f32 {
            let goal = "AAABBBCCC";
            let mut wrong = 0;
            for (g, z) in self.s.chars().zip(goal.chars()) {
                if g != z {
                    wrong += 1;
                }
            }
            1.0 - (wrong as f32 / (goal.len() as f32))
        }

        fn value(&self) -> f32 {
            if self.s == "AAABBBCCC" {1.0} else {0.0}
        }
    }

    #[test]
    fn it_works() {
        let initial = Guess{s:"D".to_owned()};
        let solution = solve(&initial);
        println!("take {:?} (h:{:?})", &solution, initial.value());
    }
}
