function initDocument() {
    let tables = document.querySelectorAll('.map_scores');
    
    // Select the table which has the biggest number of scores to be displayed
    let selected_table = tables[0];
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

    selected_table.setAttribute('show', '');
}

document.addEventListener('DOMContentLoaded', initDocument);