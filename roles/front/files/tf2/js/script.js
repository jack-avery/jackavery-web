window.onload = function() {
    handleURLChange();

    serverRefresh();
    setInterval(serverRefresh, 15000);
}

window.onpopstate = function() {
    handleURLChange();
}

function handleURLChange() {
    // get tab from hash & set
    var idx = window.location.hash.substring(1);
    tab(idx);
}

function tab(id) {
    // reset content and tabs
    var x = document.getElementsByClassName("tab");
    for (var i = 0; i < x.length; i++) {
        x[i].style.display = "none";
    }
    var x = document.getElementsByClassName("navtab");
    for (i = 0; i < x.length; i++) {
        x[i].style.animation = "navtab-deselect 0.5s linear forwards";
    }
    // get tab and nav button
    var content = document.getElementById(id+"tab");
    var tab = document.getElementById(id+"nav");
    if (!(!!content && !!tab)) { // if doesn't exist, use about
        content = document.getElementById("abouttab");
        tab = document.getElementById("aboutnav");
        id = "about";
    }
    // set animations
    content.style.display = "block";
    tab.style.animation = "navtab-select 0.5s linear forwards";
    // set tab in hash
    window.location.hash = id;
}

const listingsbody = `
<tbody>
    <tr>
        <th>players</th>
        <th>//</th>
        <th>connect</th>
        <th>//</th>
        <th>hostname</th>
        <th>//</th>
        <th>current map</th>
    </tr>
    $listings
</tbody>
`

const baselisting = `
<tr>
    <td $occupied>$players/$maxplayers</td>
    <td>//</td>
    <td><button class="promptbutton" id="$connect" onclick="getConnectString('$connect')">copy</button></td>
    <td>//</td>
    <td $protected>$hostname</td>
    <td>//</td>
    <td>$map</td>
</tr>
`

function getConnectString(connect) {
    navigator.clipboard.writeText(connect);

    var button = document.getElementById(connect);
    button.style.animation = 'none';
    button.offsetHeight; /* trigger reflow */
    button.style.animation = null;
}

async function serverRefresh() {
    servers = fetch("https://jackavery.ca/api/hosts")
    .then(response => response.json())
    .then(data => {
        container = document.getElementById("serverlistcontainer");
        let listings = "";
        let table = listingsbody;
        // construct server listings
        for (const serverinfo of data) {
            let server = baselisting;

            // set playercount to be colorful if server is occupied
            if (serverinfo.players > 0) {
                server = server.replace("$occupied", `class="tcolors"`);
            } else {
                server = server.replace("$occupied", "");
            }

            // set hostname to be grey if server is pass protected
            if (serverinfo.is_pass_protected) {
                server = server.replace("$protected", `class="grey"`);
            } else {
                server = server.replace("$protected", "");
            }

            // set remainder of fields
            for (const [key, value] of Object.entries(serverinfo)) {
                server = server.replaceAll("$"+key, value);
            }

            // append
            listings += server;
        }
        table = listingsbody.replace("$listings", listings);
        // set server listings
        container.innerHTML = table;
    })
    .catch(err => {
        console.log(err);
    })
}
