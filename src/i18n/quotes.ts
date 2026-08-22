// we don't i18n these.
export const randomQuotes = [
    "I like me better when I'm with you.", // Lauv
    "The best disinfectant is a smile.",
    "You're lovely.",
    "The cake is a lie.", // Portal 1|2
    "For the universe said I love you, because you are love.", // - Minecraft poem
    "https://www.youtube.com/watch?v=dQw4w9WgXcQ", // - Rick Astley
    "If failure is for breakfast, then what's for dinner?",
    "Be-better in stereo!", // Liv & Maddie
    "100 x 40!",
    "Tux cats are so handsome.",
    "Quizas no fue coincidencia encontrarme contigo.", // Colgando en tus manos - Carlos Baute & Marta Sánchez
    "I am Groot.", // Guardians of the Galaxy
    "Sunlight is said to be the best of disinfectants - US Supreme Court Justice Louis Brandeis", // - US Supreme Court Justice Louis Brandeis
    "I am inevitable.", // Avengers: Endgame
    "I am Iron Man.", // Avengers: Endgame
    "La gente buena no se entierran, se siembra - Pedro Capo", // La Fiesta - Pedro Capo
    "Pasito pasito, suave suavecito", // Despacito - Luis Fonsi
    "Where in the world is Tarzan?", // Carmen Sandiego
    "Silly goose.",
    "Just you & I defying gravity", // Wicked
    "Yo solo quiero, un poco de cafe", // Quiero Cafe - Jovani Vazquez
    "One day you'll leave this world behind, so live a life you will remember - Avicii", // RIP AVICII
    "Yo te miro y todo me da vueltas - Presiento | Morat", // Presiento - Morat
    "El mundo esta compuesto de atomos y lecciones de vida - Aramis", // Prof. Rafael Aramis Lopez Vargas
    "Resuelve!", // JP
    "I'm gonna reach for the stars, although they look pretty far - Tomoya Ohtani", // Sonic Colors Theme Song
    "Say Geronimo!", // Geronimo - Sheppard
    "I'm making a note here. Huge success. It's hard to overstate my satisfaction.", // Portal 2
    "We do what we must because we can.", // Portal 2
    "Loki with a cookie.",
    "Rainbows are cool!",
    "The quick brown fox jumps over the lazy dog.",
    "My rainbow beautiful special cow without scoliosis.",
    "You've got a friend in me.", // Toy Story
    "You never know. You hope for the best and make do with what you get.", // Nick Fury,
    "Vamos pa' la playa, pa' curarte el alma; Cierra la pantalla, abre la Medalla", // - Calma - Pedro Capo
];

function dayOfYear(date: Date): number {
    const start = new Date(date.getFullYear(), 0, 0);
    const diff = date.getTime() - start.getTime();
    const oneDay = 1000 * 60 * 60 * 24;
    return Math.floor(diff / oneDay);
}

// Deterministic PRNG (mulberry32) -- Math.random() can't be seeded, so this
// is what lets the shuffle below be reproducible for a given seed.
function mulberry32(seed: number) {
    return function () {
        seed |= 0;
        seed = (seed + 0x6d2b79f5) | 0;
        let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
        t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
        return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
}

function shuffleQuotes() {
    // seeded by day of the year, so the order is stable all day but
    // reshuffles day to day
    const rng = mulberry32(dayOfYear(new Date()));
    const quotes = [...randomQuotes];

    for (let i = quotes.length - 1; i > 0; i--) {
        const j = Math.floor(rng() * (i + 1));
        [quotes[i], quotes[j]] = [quotes[j], quotes[i]];
    }

    return quotes;
}

export const shuffledQuotes = shuffleQuotes();
