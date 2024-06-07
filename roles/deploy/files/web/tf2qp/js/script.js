var toggle_crits;
var toggle_spread;
var toggle_population;
var toggle_maps;
var select_region;
var increase_range;

var toggle_uncletopia;
var toggle_skial;

var search;

var lookup;
var info;

window.onload = function() {
    handleURLChange();

    let ticket = localStorage.getItem('ticket');

    if (!ticket) {
        let ticket = get_ticket();
        localStorage.setItem('ticket', ticket);
    }

    toggle_crits = document.getElementById("crits");
    toggle_spread = document.getElementById("spread");
    toggle_population = document.getElementById("population");
    toggle_maps = document.getElementById("maps");
    select_region = document.getElementById("region");
    increase_range = document.getElementById("wider");

    toggle_uncletopia = document.getElementById("uncletopia");
    toggle_skial = document.getElementById("skial");

    search = document.getElementById("search");

    lookup = {
        "crits": toggle_crits,
        "spread": toggle_spread,
        "population": toggle_population,
        "maps": toggle_maps,
        "region": select_region,
        "wider": increase_range,

        "uncletopia": toggle_uncletopia,
        "skial": toggle_skial
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
        },
        "skial": {
            "crits": true,
            "spread": true,
            "maps": true,
            "regions": [
                "naw",
                "nae"
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
    search.disabled = !allow_search;

    save_preferences();
}

function find_game() {
    alert("wip");
}