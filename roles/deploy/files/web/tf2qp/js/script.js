var crits_pref;
var spread_pref;
var alltalk_pref;
var toggle_maps;
var toggle_full;
var toggle_empty;

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
var search_button_border;
var im_feeling_lucky_button;
var im_feeling_lucky_button_border;
var error_text;

var server_list_div;
var query_servers;

var lookup;
var regions_list;
var networks_list;

window.onload = function() {
    handleURLChange();

    crits_pref = document.getElementById("crits");
    spread_pref = document.getElementById("spread");
    alltalk_pref = document.getElementById("alltalk");
    toggle_community_maps = document.getElementById("toggle_community_maps");
    toggle_empty = document.getElementById("toggle_empty");
    toggle_full = document.getElementById("toggle_full");

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
    search_button_border = document.getElementById("search_border");
    im_feeling_lucky_button = document.getElementById("im_feeling_lucky");
    im_feeling_lucky_button_border = document.getElementById("lucky_border");
    error_text = document.getElementById("error_text");

    server_list_div = document.getElementById("server_list");

    // network internal IDs and their formal name
    networks_list = {
        "uncletopia": "Uncletopia",
        "skial": "Skial",
        "swishcast": "SwishCast",
        "oprah": "Oprah's Petrol Station",
        "casualtf": "Casual.TF"
    }

    // global lookup table for id and corresponding element
    lookup = {
        "crits": crits_pref,
        "spread": spread_pref,
        "alltalk": alltalk_pref,
        "maps": toggle_community_maps,
        "toggle_empty": toggle_empty,
        "toggle_full": toggle_full,

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

const NETWORK = `<label><input type="checkbox" id="%ID" onclick="update_form()">%NAME</label><br>`
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

        try {
            switch (lookup[item].type) {
                case "checkbox":
                    lookup[item].checked = (value == "true");
                    break;
                case "select-one":
                    lookup[item].value = value;
                    break;
            }
        } catch {
            localStorage.clear();
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

    for (network in networks_list) {
        if (lookup[network].checked) {
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

    if (disable_search) {
        search_button_border.classList.add("disabled");
        im_feeling_lucky_button_border.classList.add("disabled");
    } else {
        search_button_border.classList.remove("disabled");
        im_feeling_lucky_button_border.classList.remove("disabled");
    }

    save_preferences();
}

// search games
// instead of returning the games, this shunts it into the global `query_servers`
function search(callback) {
    let crits = crits_pref.value;
    let spread = spread_pref.value;
    let allow_empty = toggle_empty.checked;
    let allow_full = toggle_full.checked;
    let alltalk = alltalk_pref.value;
    let allow_community_maps = toggle_community_maps.checked;

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

    let query_url = `https://jackavery.ca/api/hosts/${regions}/${networks}/${allow_empty}/${allow_full}/${allow_community_maps}/${crits}/${spread}/${alltalk}`;
    console.log(query_url);
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

const ENTRIES = `<table>
    <thead>
        <tr>
            <td>Hostname</td>
            <td>Map</td>
            <td>Players</td>
            <td>IP</td>
            <td>Connect</td>
        </tr>
        %ENTRIES
    </thead>
</table>`
const ENTRY = `
<tr %ODD>
    <td>%HOSTNAME</td>
    <td class="map">%MAP</td>
    <td %STATE>%PLAYERS / %MAXPLAYERS</td>
    <td><button onclick="navigator.clipboard.writeText('%IP')">Copy</a></td>
    <td>%CONNECT</td>
</tr>
`
const CONNECT = `
<a target="_blank" href="steam://connect/%IP"><button class="join_button">Join!</button></a>
`
const CONNECT_FULL = `
<button disabled>Full</button>
`

// populate the list. if "ip mode" is enabled it only shows IP addresses
function populate_server_list() {
    if (!query_servers || query_servers.matches == 0) {
        return
    }

    let entry_list = '';
    let odd = false;
    for (host in query_servers.hosts) {
        host = query_servers.hosts[host];
        odd = !odd;
        
        let odd_class = '';
        if (odd) {
            odd_class = `class="odd"`
        }

        let state = '';
        let connect = CONNECT.replaceAll("%IP", host.ip);
        if (host.players >= host.maxplayers) {
            connect = CONNECT_FULL;
            state = `class="full"`
        } else if (host.players == 0) {
            state = `class="empty"`
        }

        entry_list += ENTRY
            .replaceAll("%ODD", odd_class)
            .replaceAll("%IP", host.ip)
            .replaceAll("%HOSTNAME", host.hostname)
            .replaceAll("%MAP", host.map)
            .replaceAll("%PLAYERS", host.players)
            .replaceAll("%MAXPLAYERS", host.maxplayers)
            .replaceAll("%STATE", state)
            .replaceAll("%CONNECT", connect)
    }
    server_list_div.innerHTML = ENTRIES.replaceAll("%ENTRIES", entry_list);
}

// search & join a random server
function im_feeling_lucky() {
    let server_ip = query_servers.hosts[Math.floor(Math.random() * query_servers.matches)]["ip"];
    window.open(`steam://connect/${server_ip}`, '_blank').focus();
}
