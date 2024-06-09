var crits_pref;
var spread_pref;
var toggle_population;
var toggle_maps;
var select_region;
var increase_range;

var toggle_uncletopia;
var toggle_skial;

var find_game_button;

var lookup;
var info;

window.onload = function() {
    handleURLChange();

    crits_pref = document.getElementById("crits");
    spread_pref = document.getElementById("spread");
    toggle_population = document.getElementById("population");
    toggle_maps = document.getElementById("maps");
    select_region = document.getElementById("region");
    increase_range = document.getElementById("wider");

    toggle_uncletopia = document.getElementById("uncletopia");
    toggle_skial = document.getElementById("skial");

    find_game_button = document.getElementById("find_game");

    lookup = {
        "crits": crits_pref,
        "spread": spread_pref,
        "population": toggle_population,
        "maps": toggle_maps,
        "region": select_region,
        "wider": increase_range,

        "uncletopia": toggle_uncletopia
    }

    info = {
        "uncletopia": {
            "crits": false,
            "spread": false,
            "maps": true,
            "regions": [
                "naw",
                "nae",
                "sa",
                "euw",
                "eune",
                "as",
                "oce"
            ]
        }
    }

    load_preferences();
    update_form();
}

function load_preferences() {
    for (item in lookup) {
        switch (lookup[item].type) {
            case "checkbox":
                lookup[item].checked = (localStorage.getItem(item) == "true");
                break;
            case "select-one":
                lookup[item].value = localStorage.getItem(item);
                break;
        }
    }
}

function save_preferences() {
    for (item in lookup) {
        switch (lookup[item].type) {
            case "checkbox":
                localStorage.setItem(item, lookup[item].checked);
                break;
            case "select-one":
                localStorage.setItem(item, lookup[item].value);
                break;
        }
    }
}

function update_form() {
    for (network in info) {
        if (
            (lookup.crits.value == "1" && !info[network].crits) || // crits, pref no
            (lookup.spread.value == "1" && !info[network].spread) || // spread, pref no
            (lookup.crits.value == "0" && info[network].crits) || // nocrits, pref
            (lookup.spread.value == "0" && info[network].spread) || // nospread, pref
            (lookup.maps.value == "1" && !info[network].maps) || // no community maps servers, pref
            (!info[network].regions.includes(lookup.region.value)) // no servers in region
        ) {
            lookup[network].checked = false;
            lookup[network].disabled = true;
            continue;
        }

        lookup[network].disabled = false;
    }

    let allow_search = false;
    for (network in info) {
        if (lookup[network].checked) {
            allow_search = true;
        }
    }
    find_game_button.disabled = !allow_search;

    save_preferences();
}

function find_games() {
    let crits = crits_pref.value;
    let spread = spread_pref.value;
    let allow_empty = toggle_population.value == '0';
    let allow_community_maps = toggle_maps == '1';

    let region = select_region.value;
    let extend = increase_range.checked;

    let networks = '';
    for (network in info) {
        if (lookup[network].checked) {
            networks += network
        }
    }

    let query_url = `https://jackavery.ca/api/hosts/${region}/${extend}/${allow_community_maps}/${allow_empty}/${crits}/${spread}/${networks}`

    //TODO: add favorites export and "I'm feeling lucky"
    // fetch(query_url)
    // .then(response => response.json())
    // .then(
    //     return response
    // )

    window.open(query_url, '_blank').focus();
}
