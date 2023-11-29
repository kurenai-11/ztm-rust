// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State> {
    id: i32,
    state: State,
}

impl<State> Luggage<State> {
    fn transition<NewState>(self, new_state: NewState) -> Luggage<NewState> {
        Luggage {
            id: self.id,
            state: new_state,
        }
    }
}

struct CheckIn;
impl Luggage<CheckIn> {
    fn new(id: i32) -> Self {
        Self { id, state: CheckIn }
    }

    fn load(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}
struct OnLoading;
impl Luggage<OnLoading> {
    fn offload(self) -> Luggage<Offloading> {
        self.transition(Offloading)
    }
}
struct Offloading;
impl Luggage<Offloading> {
    fn prepare(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}
struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn pick_up(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}
struct EndCustody;
impl Luggage<EndCustody> {
    fn congratulate(self) {
        println!("package handled.")
    }
}

fn main() {
    Luggage::new(131881)
        .load()
        .offload()
        .prepare()
        .pick_up()
        .congratulate();
}
