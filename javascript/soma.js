const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question("", function(A) {
  rl.question("", function(B) {
    A = parseInt(A);
    B = parseInt(B);
    const soma = A + B;

    console.log(`SOMA = ${soma}`);
    
    rl.close();
  });
});