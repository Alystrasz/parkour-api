const fs = require('node:fs');


function get_players_count(maps_path, configurations_path, scores_path, event_id) {
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


    // Retrieve configurations from map IDs
    const configuration_ids = [];
    if (!fs.existsSync(configurations_path)) {
        console.error(`Configurations file not found (input path was "${configurations_path}").`);
        process.exit(10);
    }
    const configurations_data = fs.readFileSync(configurations_path, 'utf8');
    const configurations = JSON.parse(configurations_data);
    for (const mapId of event_maps_ids) {
        const mapConfig = configurations[mapId];
        if (mapConfig === undefined) {
            console.error(`No configuration found (map id was "${mapId}").`);
            process.exit(11);
        }
        configuration_ids.push(...mapConfig.map(c => c.id));
    }
    console.log(`Retrieved configuration IDs: ['${configuration_ids.join("', '")}']`);


    // Retrieve scores from configuration IDs
    const player_names = [];
    if (!fs.existsSync(scores_path)) {
        console.error(`Scores file not found (input path was "${scores_path}").`);
        process.exit(3);
    }
    const data = fs.readFileSync(scores_path, 'utf8');
    const scores = JSON.parse(data);
    for (const config_id of configuration_ids) {
        const config_scores = scores[config_id];
        if (config_scores === undefined) {
            console.error(`Scores not found for config id=${config_id}.`);
            process.exit(4);
        }
        // Only add player name if it wasn't previously registered
        config_scores.forEach(entry => {
            if (!player_names.includes(entry.name)) {
                player_names.push(entry.name);
            }
        });
    }

    // Final result
    console.log(`Unique player count: ${player_names.length}`);
}


// Main
if (process.argv.length !== 6) {
    console.error(`Incorrect format. Expected use:\n\tnode [path/to/get_players_count.js] [path/to/maps.json] [path/to/configurations.json] [path/to/scores.json] [event_id]`);
    return;
}
get_players_count(process.argv[2], process.argv[3], process.argv[4], process.argv[5]);
