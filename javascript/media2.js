const readline = require('readline')

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.question("", function(A) {
    rl.question("", function(B) {
        rl.question("", function(C) {
            A = parseFloat(A);
            B = parseFloat(B);
            C = parseFloat(C);
            let media = ((A *2) + (B * 3) + (C *5)) / 10
            let format = media.toFixed(1)
            console.log(`MEDIA = ${format}`)
        })
        
    })
})