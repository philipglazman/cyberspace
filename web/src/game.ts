import { SuiClient, getFullnodeUrl } from "@mysten/sui.js/client";

const GAME_OBJECT = "0x813002af527d0803c3bd3a96ba3fc59a829f2e84d21d985cfed64464da57d5d9";

const GAME_PROGRAM = "0x5f234782d0d7fcb5412aafca6e87be1e6b2c67383566f2f4c499bc12ddafb385"

interface GameFields {
    name: string;
    age: number;
    city: string;
}

export async function GetGameRandomness(client: SuiClient) {
    const obj = await GetGameObject(client);
    return obj.fields.seed;
}

export async function GetGameObject(client: SuiClient) {
    const obj = await client.getObject({
        id: GAME_OBJECT,
        options: {
            showContent: true,
        },
    });
    let move_object: any = obj.data?.content;

    return move_object;
}

export function CallTargetForPlayerRegistration(): string {
    return `${GAME_OBJECT}::Game::enter_game`;
}

export function GameObjectID(): string {
    return GAME_OBJECT;
}

export function GameProgramID(): string {
    return GAME_PROGRAM;
}

export async function IsUserPartOfGame(client: SuiClient, address: String): Promise<Boolean> {
    const obj = await GetGameObject(client);
    let players = obj.fields.players;
    if (players.players.includes(address)) {
        return true
    };
    // TODO: test this.
    return false
}