module cyberspace::Game {
    use sui::object::UID;
    use sui::random::{Random, new_generator};
    use sui::vec_set::{Self, VecSet};

    public struct Game has key {
        id: UID,
        seed: u32,
        players: Registry
    }

    public struct Registry has key, store {
        id: UID,
        players: VecSet<address>,
    }

    fun init_registry(ctx: &mut TxContext): Registry {
        let id = object::new(ctx);
        Registry {
            id: id,
            players: vec_set::empty(),
        }
    }

    public struct GameOwnerCap has key {
        id: UID
    }

    fun init(ctx: &mut TxContext) {
        transfer::transfer(GameOwnerCap {
            id: object::new(ctx),
        }, ctx.sender())
    }

    fun create_random_map(r: &Random, ctx: &mut TxContext): u32 {
        let mut generator = new_generator(r, ctx);
        let seed = generator.generate_u32();
        seed
    }

    public fun create_game(_cap: &GameOwnerCap, r: &Random, ctx: &mut TxContext) {
        let seed = create_random_map(r, ctx);

        transfer::share_object(Game {
            id: object::new(ctx),
            seed: seed,
            players: init_registry(ctx),
        })
    }

    public entry fun enter_game(game: &mut Game, ctx: &mut TxContext) { 
        let player = ctx.sender();
        vec_set::insert(&mut game.players.players, player);
    }
}

