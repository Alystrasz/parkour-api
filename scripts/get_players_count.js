const fs = require('node:fs');

function get_players_count(maps_path, scores_path, event_id) {
    // Retrieve map IDs from event ID
    if (!fs.existsSync(maps_path)) {
        console.error(`Maps file not found (input path was "${maps_path}").`);
        process.exit(1);
    }
    const maps_data = fs.readFileSync(maps_path, 'utf8');
    const maps = JSON.parse(maps_data);
    const event_maps = maps[event_id];
    if (event_maps === undefined) {
        console.error(`Event not found (input id was "${event_id}").`);
        process.exit(2);
    }
    const event_maps_ids = event_maps.map(map => map.id);
    console.log(`Retrieved map IDs: ['${event_maps_ids.join("', '")}']`);

    const player_names = [];

    // Retrieve players info from map IDs
    if (!fs.existsSync(scores_path)) {
        console.error(`Scores file not found (input path was "${scores_path}").`);
        process.exit(3);
    }
    const data = fs.readFileSync(scores_path, 'utf8');
    const scores = JSON.parse(data);
    for (const map_id of event_maps_ids) {
        const map_scores = scores[map_id];
        if (map_scores === undefined) {
            console.error(`Map scores not found for map id=${map_id}.`);
            process.exit(4);
        }
        // Only add player name if it wasn't previously registered
        map_scores.forEach(entry => {
            if (!player_names.includes(entry.name)) {
                player_names.push(entry.name);
            }
        });
    }

    // Final result
    console.log(`Unique player count: ${player_names.length}`);
}

// Main
if (process.argv.length !== 5) {
    console.error(`Incorrect format. Expected use:\n\tnode [path/to/get_players_count.js] [path/to/events.json] [path/to/scores.json] [event_id]`);
    return;
}
get_players_count(process.argv[2], process.argv[3], process.argv[4]);
