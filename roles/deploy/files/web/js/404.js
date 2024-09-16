let VIDEOS = [
    "TKfS5zVfGBc", // 009 Sound System - Dreamscape
    "ZZ5LpwO-An4", // HEYYEYAAEYAAAEYAEYAA
    "5K7Frc2lTI8", // Kitsune^2 - Rainbow Tylenol
    "QDvvt1kmL1Q", // Kitsune^2 - Rock my Emotions
    "-RFunvF0mDw", // Nobody Here
    "yMYj-UwdpA0", // Checkpoint
    "8d44ykdKvCw", // Basshunter - DotA
    "I8sUC-dsW8A", // Tally Hall - Ruler of Everything
    "dLrdSC9MVb4", // Tally Hall - Turn The Lights Off
    "EGXPAoyP_cg", // Chris Christodoulou - The Rain Formerly Known as Purple (Risk of Rain 2 OST)
    "103F1YbbSv0", // Chris Christodoulou - The Face of the Deep (Risk of Rain 2 OST)
    "iKnwVvXkWq0", // Lena Raine & 2 Mello - Mirror Temple (Mirror Magic Mix) (Celeste OST)
    "GISnTECX8Eg", // ??? - Space Asshole (SS13 OST)
    "FBdoJppGvxw", // From Grotto - Torni (Noita OST)
    "Q7eJg7hRvqE", // Valve Orchestra - Upgrade Station (Unused?)
    "RBRlXsSXzCg", // Valve Orchestra - Red Bread (Expiration Date Intro)
    "FEiTgU-hM5k", // Keith Power - This Is What You Are (Warframe OST)
    "7Kle8FixygY", // Aaron Cherof - Precipice (Minecraft OST, Tanger Remix)
    "kK81m-A3qpU", // Lena Raine - Otherside (Minecraft OST)
    "PZAM-_5h4QE", // 65daysofstatic - Supermoon (No Man's Sky OST)
    "rbxL5BVEkRs", // Lemon Demon (Neil Cicierega) - Touch-Tone Telephone
    "hPMnIymc3Cs", // Lemon Demon (Neil Cicierega) - Spiral of Ants
    "_LUFMHvvNt4", // Exyl - Together Forever (ft. Rythm)
    "vatcanQQJvQ", // xi - Blue Zenith
    "vZyenjZseXA", // Virtual Riot - Idols
]

let PROMPTS = [
    "you can see the list in the javascript source, by the way",
    "!rtd",
    "!rtv",
    "there's more...",
    "got any others?",
    "give me another one!",
    "haven't i seen this one already?",
    "how many of these are there?",
    "bring me another!",
]

window.onload = function() {
    var content = document.getElementById("abouttab");
    var tab = document.getElementById("aboutnav");
    content.style.display = "block";
    tab.style.animation = "navtab-select 0.5s linear forwards";

    getNext();
};

function getNext() {
    var video = Math.floor(Math.random() * VIDEOS.length);
    document.getElementById('404video').src = "https://www.youtube.com/embed/" + VIDEOS[video];

    var button = document.getElementById('prompt');
    var prompt = Math.floor(Math.random() * PROMPTS.length);
    button.innerHTML = PROMPTS[prompt];
    button.style.animation = 'none';
    button.offsetHeight; /* trigger reflow */
    button.style.animation = null;
}