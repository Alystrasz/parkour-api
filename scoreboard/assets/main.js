function initDocument() {
    let tables = document.querySelectorAll('.result_scores');
    let selected_table = tables[0];
    let noTableFound = true;

    // Check if a config argument was passed
    const url = new URL(window.location);
    const searchParams = url.searchParams;
    if (searchParams.has('route')) {
        // Retrieve config id through selection menu item (a bit hacky I know)
        let routeName = searchParams.get('route');
        const li = document.querySelector(`li[result_id=PK_${routeName}]`);

        if (li !== null) {
            const id = li.getAttribute('result_id').substring(3); // Remove "PK_" prefix
            const matchingTable = document.querySelector(`table[id="result_${id}"]`);
            selected_table = matchingTable;
            noTableFound = false;
        } else {
            console.warn(`Could not find route with name=${routeName}, defaulting to the most populated table.`);
        }
    }

    // Select the table which has the biggest number of scores to be displayed
    if (noTableFound) {
        let count = 0;

        for (let i=0; i<tables.length; i++) {
            const table = tables[i];
            const body = table.querySelector('tbody');
            const childrenCount = body.children.length;

            if (childrenCount > count) {
                count = childrenCount;
                selected_table = table;
            }
        }
    }

    // Retrieve map name through selection item (a bit hacky I know)
    const li = document.querySelector(`li[result_id="PK_${selected_table.getAttribute('result_id')}"]`);
    displayTable(selected_table.id, li.dataset.mapName);

    // Set up route names in the route selector
    const routeItems = document.querySelectorAll('#resultsList li');
    for (const item of routeItems) {
        item.innerText = `${item.dataset.configName} (${getMapName(item.dataset.mapName)})`;
    }
}

function displayTable(tableId, mapName) {
    // Hide all tables
    let tables = document.querySelectorAll('.result_scores');
    for (let i=0; i<tables.length; i++) {
        tables[i].removeAttribute('show');
    }

    // Show table
    let table = document.querySelector('#' + tableId);
    table.setAttribute('show', '');

    // Update map card
    document.getElementById('mapName').innerText = getMapName(mapName);
    document.getElementById('routeSelectorImage').setAttribute('src', 'assets/img/maps/' + mapName + '.webp')
}

function getMapName(map) {
    switch (map) {
        case 'mp_angel_city':
            return 'Angel City';
        case 'mp_black_water_canal':
            return 'Black Water Canal';
        case 'mp_coliseum':
            return 'Coliseum';
        case 'mp_colony02':
            return 'Colony';
        case 'mp_complex03':
            return 'Complex';
        case 'mp_crashsite3':
            return 'Crash Site';
        case 'mp_drydock':
            return 'Drydock';
        case 'mp_eden':
            return 'Eden';
        case 'mp_forwardbase_kodai':
            return 'Forwardbase Kodai';
        case 'mp_glitch':
            return 'Glitch';
        case 'mp_grave':
            return 'Boomtown';
        case 'mp_homestead':
            return 'Homestead';
        case 'mp_relic02':
            return 'Relic';
        case 'mp_rise':
            return 'Rise';
        case 'mp_thaw':
            return 'Exoplanet';
        case 'mp_wargames':
            return 'War Games';
        default:
            return 'Unknown';
    }
}

function toggleResultsListDisplay() {
    const list = document.getElementById('resultsList');
    list.toggleAttribute('show');
}

document.addEventListener('DOMContentLoaded', initDocument);
document.body.addEventListener('click', function() {
    const list = document.getElementById('resultsList');
    list.toggleAttribute('show', false);
}, true);