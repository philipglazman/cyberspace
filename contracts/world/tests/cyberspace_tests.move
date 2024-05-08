
#[test_only]
module world::world_tests {
    // uncomment this line to import the module
    use sui::test_scenario;
    use world::world;
    use sui::random;
    use sui::random::{Random};
    use sui::test_scenario::{ctx, take_from_sender, next_tx, return_to_sender};
    use std::debug;

    const ENotImplemented: u64 = 0;

    #[test]
    fun test_world() {
        let user0 = @0x0;
        let user1 = @0x1;
        let mut scenario_val = test_scenario::begin(user0);
        let scenario = &mut scenario_val;

        random::create_for_testing(ctx(scenario));
        test_scenario::next_tx(scenario, user0);

        let mut random_state = test_scenario::take_shared<Random>(scenario);
        
        
        let res = world::new_world_seed(&random_state, ctx(scenario));
        debug::print(&res);
        test_scenario::return_shared(random_state);
        test_scenario::end(scenario_val);
        // pass
    }

    // #[test, expected_failure(abort_code = world::world_tests::ENotImplemented)]
    // fun test_world_fail() {
    //     abort ENotImplemented
    // }
}
