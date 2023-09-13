const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question("", function(A) {
  rl.question("", function(B) {
    A = parseFloat(A);
    B = parseFloat(B);
    const media = ((A * 3.5) + (B * 7.5)) / 11
    let casasDecimais = media.toFixed(5)

    console.log(`MEDIA = ${casasDecimais}`);
    
    rl.close();
  });
});
