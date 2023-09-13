const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.question("", function(raio) {
    raio = parseFloat(raio);
    let pi = 3.14159;
    let area = pi * Math.pow(raio, 2);
    let formattedArea = area.toFixed(4);
    console.log(`A=${formattedArea}`);
    rl.close();
});