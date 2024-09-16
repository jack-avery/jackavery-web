window.onload = function() {
    handleURLChange();
};

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
};