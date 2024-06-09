var crits_pref;
var spread_pref;
var population_pref;
var toggle_maps;
var select_region;
var increase_range;

var toggle_uncletopia;
var toggle_skial;

var search_button;
var im_feeling_lucky_button;
var error_text;

var server_list_div;
var server_list_ip_mode;
var query_servers;

var lookup;
var info;

window.onload = function() {
    handleURLChange();

    crits_pref = document.getElementById("crits");
    spread_pref = document.getElementById("spread");
    population_pref = document.getElementById("population");
    toggle_maps = document.getElementById("maps");
    select_region = document.getElementById("region");
    increase_range = document.getElementById("wider");

    toggle_uncletopia = document.getElementById("uncletopia");
    toggle_skial = document.getElementById("skial");

    search_button = document.getElementById("search");
    im_feeling_lucky_button = document.getElementById("im_feeling_lucky");
    error_text = document.getElementById("error_text");

    server_list_div = document.getElementById("server_list");
    server_list_ip_mode = document.getElementById("ip_mode");

    // global lookup table for id and corresponding element
    lookup = {
        "crits": crits_pref,
        "spread": spread_pref,
        "population": population_pref,
        "maps": toggle_maps,
        "region": select_region,
        "wider": increase_range,
        "ip_mode": server_list_ip_mode,

        "uncletopia": toggle_uncletopia
    }

    // networks info
    info = {
        "uncletopia": {
            "regions": [
                "naw",
                "nac",
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

// load query parameters from storage
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

// save pquery parameters
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

// update the search and feeling lucky buttons depending on whether
// the current user selection is valid
function update_form() {
    for (network in info) {
        if (
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
    search_button.disabled = !allow_search;
    im_feeling_lucky_button.disabled = !allow_search;

    save_preferences();
}

// search games
// instead of returning the games, this shunts it into the global `query_servers`
function search(callback) {
    let crits = crits_pref.value;
    let spread = spread_pref.value;
    let pop = population_pref.value;
    let allow_community_maps = toggle_maps.value == '1';

    let region = select_region.value;
    let extend = increase_range.checked;

    let networks = '';
    for (net in info) {
        if (lookup[net].checked) {
            networks += net;
        }
    }

    let query_url = `https://jackavery.ca/api/hosts/${region}/${extend}/${allow_community_maps}/${pop}/${crits}/${spread}/${networks}`;
    fetch(query_url)
    .then(response => response.json())
    .then(response => {
        if (response.code != 200) {
            throw Error(response.message);
        }
        if (response.matches == 0) {
            throw Error("your query didn't match any servers :(");
        }
        error_text.innerHTML = "";
        query_servers = response;
        callback();
    })
    .catch(err => {
        server_list_div.innerHTML = '';
        error_text.innerHTML = err;
        console.log(err);
    });
}

var ENTRIES_IP_MODE = `<p>click on an IP address to copy the connect info</p><br><p>%ENTRIES</p>`
var ENTRY_IP_MODE = `<a onclick="navigator.clipboard.writeText('%IP')">%IP</a><br>`

var ENTRIES = `<p>click on an IP address to copy the connect info</p><br>%ENTRIES`
var ENTRY = `
<h3>%HOSTNAME (%NETWORK)</h3>
<p>
<a onclick="navigator.clipboard.writeText('connect %IP')">%IP</a><br>
%STATUS<br>
</p>
<div class="small-vertical-divider"></div>
`

// populate the list. if "ip mode" is enabled it only shows IP addresses
function populate_server_list() {
    if (!query_servers || query_servers.matches == 0) {
        return
    }

    if (server_list_ip_mode.checked) {
        let entry_list = '';
        for (host in query_servers.hosts) {
            host = query_servers.hosts[host];
            entry_list += ENTRY_IP_MODE.replaceAll("%IP", host.ip);
        }
        server_list_div.innerHTML = ENTRIES_IP_MODE.replaceAll("%ENTRIES", entry_list);
    } else {
        let entry_list = '';
        for (host in query_servers.hosts) {
            host = query_servers.hosts[host];

            let status = `${host.players}/${host.maxplayers} on ${host.map}`
            if (host.players >= host.maxplayers || host.players == 0) {
                status = `<span class="full_or_empty">${status}</span>`
            }

            entry_list += ENTRY
                .replaceAll("%IP", host.ip)
                .replaceAll("%HOSTNAME", host.hostname)
                .replaceAll("%NETWORK", host.network)
                .replaceAll("%STATUS", status)
        }
        server_list_div.innerHTML = ENTRIES.replaceAll("%ENTRIES", entry_list);
    }
}

// search & join a random server
function im_feeling_lucky() {
    let server_ip = query_servers.hosts[Math.floor(Math.random() * query_servers.matches)]["ip"];
    let entry_list = '';
    entry_list += ENTRY_IP_MODE.replaceAll("%IP", server_ip);
    server_list_div.innerHTML = ENTRIES_IP_MODE.replaceAll("%ENTRIES", entry_list);
}
