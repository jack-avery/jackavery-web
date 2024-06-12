var crits_pref;
var spread_pref;
var population_pref;
var alltalk_pref;
var toggle_maps;

var toggle_naw;
var toggle_nac;
var toggle_nae;
var toggle_sa;
var toggle_euw;
var toggle_eune;
var toggle_as;
var toggle_oce;

var networks_container;

var search_button;
var im_feeling_lucky_button;
var error_text;

var server_list_div;
var server_list_ip_mode;
var query_servers;

var lookup;
var regions_list;
var networks_list;

window.onload = function() {
    handleURLChange();

    crits_pref = document.getElementById("crits");
    spread_pref = document.getElementById("spread");
    population_pref = document.getElementById("population");
    alltalk_pref = document.getElementById("alltalk");
    toggle_maps = document.getElementById("maps");

    toggle_naw = document.getElementById("naw");
    toggle_nac = document.getElementById("nac");
    toggle_nae = document.getElementById("nae");
    toggle_sa = document.getElementById("sa");
    toggle_euw = document.getElementById("euw");
    toggle_eune = document.getElementById("eune");
    toggle_as = document.getElementById("as");
    toggle_oce = document.getElementById("oce");

    networks_container = document.getElementById("networks_container");

    search_button = document.getElementById("search");
    im_feeling_lucky_button = document.getElementById("im_feeling_lucky");
    error_text = document.getElementById("error_text");

    server_list_div = document.getElementById("server_list");
    server_list_ip_mode = document.getElementById("ip_mode");

    // network internal IDs and their formal name
    networks_list = {
        "uncletopia": "uncletopia",
        "skial": "skial",
        "swishcast": "swishcast",
        "oprah": "oprah's petrol station",
        "casualtf": "casual.tf"
    }

    // global lookup table for id and corresponding element
    lookup = {
        "crits": crits_pref,
        "spread": spread_pref,
        "population": population_pref,
        "alltalk": alltalk_pref,
        "maps": toggle_maps,

        "ip_mode": server_list_ip_mode,

        "naw": toggle_naw,
        "nac": toggle_nac,
        "nae": toggle_nae,
        "sa": toggle_sa,
        "euw": toggle_euw,
        "eune": toggle_eune,
        "as": toggle_as,
        "oce": toggle_oce,
    }

    regions_list = [
        "naw",
        "nac",
        "nae",
        "sa",
        "euw",
        "eune",
        "as",
        "oce"
    ]

    populate_network_list();
    load_preferences();
    update_form();
}

const NETWORK = `<label><input type="checkbox" id="%ID" onclick="update_form()">%NAME</label>`
function populate_network_list() {
    let networks = '';
    for (network in networks_list) {
        networks += NETWORK
            .replaceAll("%ID", network)
            .replaceAll("%NAME", networks_list[network])
    }
    networks_container.innerHTML = networks;
    for (network in networks_list) {
        lookup[network] = document.getElementById(network);
    }
}

// load query parameters from storage
function load_preferences() {
    for (item in lookup) {
        let value = localStorage.getItem(item);
        if (!value) {
            continue;
        }

        switch (lookup[item].type) {
            case "checkbox":
                lookup[item].checked = (value == "true");
                break;
            case "select-one":
                lookup[item].value = value;
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
    let region_selected = false;
    let network_selected = false;

    for (i in networks_list) {
        if (lookup[networks_list[i]].checked) {
            network_selected = true;
            break;
        }
    }
    for (i in regions_list) {
        if (lookup[regions_list[i]].checked) {
            region_selected = true;
            break;
        }
    }

    let disable_search = (!network_selected || !region_selected);

    search_button.disabled = disable_search;
    im_feeling_lucky_button.disabled = disable_search;

    save_preferences();
}

// search games
// instead of returning the games, this shunts it into the global `query_servers`
function search(callback) {
    let crits = crits_pref.value;
    let spread = spread_pref.value;
    let pop = population_pref.value;
    let alltalk = alltalk_pref.value;
    let allow_community_maps = toggle_maps.value == '1';

    let regions = [];
    for (i in regions_list) {
        if (lookup[regions_list[i]].checked) {
            regions.push(regions_list[i]);
        }
    }
    regions = regions.join(":");

    let networks = [];
    for (network in networks_list) {
        if (lookup[network].checked) {
            networks.push(network);
        }
    }
    networks = networks.join(":");

    let query_url = `https://jackavery.ca/api/hosts/${regions}/${allow_community_maps}/${pop}/${crits}/${spread}/${alltalk}/${networks}`;
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

var ENTRIES_IP_MODE = `<p>click on an IP address to copy the address</p><br><p>%ENTRIES</p>`
var ENTRY_IP_MODE = `<a onclick="navigator.clipboard.writeText('%IP')">%IP</a> - %HOSTNAME<br>`

var ENTRIES = `<p>click on an IP address to copy the connect info</p><br>%ENTRIES`
var ENTRY = `
<h3><a class="join" href="steam://connect/%IP">(join)</a> %HOSTNAME</h3>
<p>
<a onclick="navigator.clipboard.writeText('connect %IP')">%IP</a><br>
%STATUS<br>
</p>
<div class="small-vertical-divider"></div>
`

function change_ip_mode() {
    populate_server_list();
    save_preferences();
}

// populate the list. if "ip mode" is enabled it only shows IP addresses
function populate_server_list() {
    if (!query_servers || query_servers.matches == 0) {
        return
    }

    if (server_list_ip_mode.checked) {
        let entry_list = '';
        for (host in query_servers.hosts) {
            host = query_servers.hosts[host];
            entry_list += ENTRY_IP_MODE
                .replaceAll("%IP", host.ip)
                .replaceAll("%HOSTNAME", host.hostname);
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
                .replaceAll("%STATUS", status)
        }
        server_list_div.innerHTML = ENTRIES.replaceAll("%ENTRIES", entry_list);
    }
}

// search & join a random server
function im_feeling_lucky() {
    let server_ip = query_servers.hosts[Math.floor(Math.random() * query_servers.matches)]["ip"];
    window.open(`steam://connect/${server_ip}`, '_blank').focus();
}
