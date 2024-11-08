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
    "rfUeWe7u364", // Tally Hall - Hymn for a Scarecrow
    "EGXPAoyP_cg", // Chris Christodoulou - The Rain Formerly Known as Purple (Risk of Rain 2 OST)
    "103F1YbbSv0", // Chris Christodoulou - The Face of the Deep (Risk of Rain 2 OST)
    "iKnwVvXkWq0", // Lena Raine & 2 Mello - Mirror Temple (Mirror Magic Mix) (Celeste OST)
    "GISnTECX8Eg", // Chris Remo - Space Asshole
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
    "nkLDh-UNqGI", // Raccy - Level Up
    "vIFJODP_zTE", // Ludicin - Signal
    "xMNRS5_eCT8", // INZO, LSDREAM - Faceplant on the Galaxy
    "ej7-BS1i35Y", // INZO - Earth Magic (ft. Elohim)
    "KEwe13VIfxk", // KORDHELL - SCOPIN
    "DwiEvPMANa8", // FEX - Subways of Your Mind (1983)
    "k1BneeJTDcU", // Bo Burnham - Welcome to the Internet
    "c69eHlQrKaY", // Molchat Doma - Kletka
    "8ravZ03m55A", // Kumi Tanioka - pokopoko
    "lCaun_EiJZQ", // Rolobi - UNDO UNDO
    "_c2NS5Om998", // KORDHELL - MURDER PLOT
    "DI-wpzVAtH4", // Infected Mushroom - Zazim Beyhad (We Move Together) (ft May Sfadia)
    "pnfaeE47zxQ", // Infected Mushroom - Black Velvet (feat. Ninet Tayeb)
    "6v98TpTscaw", // Tanger - BIKE
    "9STiQ8cCIo0", // Kenet & Rez - Unreal Super Hero 3
    "jwhSUmN5zTk", // curgney gurgney - Silly Billy
    "Tds16xhPdT0", // Darius + Rotteen - Mars Needs Ravers
    "suisIF4hwyw", // Dion Timmer - Shiawase (VIP)
    "cG8p6B4-Dmc", // Romos - Magic Touch (30+ songs mashup)
    "WNLBSarUluo", // Camellia - Night Raid with a Dragon
    "BE05mKyjehg", // Camellia - SECRET BOSS
    "hM7Eh0gGNKA", // SIAMES - Summer Nights
    "g2i-u-9A7UA", // late night drive home - Stress Relief
    "ausqr8vTMgc", // t+pazolite - You are the Miserable (Laur Remix)
    "FQnD4rt8Vog", // peak
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
