import * as readline from 'readline';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});
let numeroFuncionario: number;
let horasTrabalhadas: number;
let valorPorHora: number;

function calcularSalario(): void {
    rl.question('', (inputNumero: string) => {
        numeroFuncionario = parseInt(inputNumero)
        rl.question('', (inputHoras: string) => {
            horasTrabalhadas = parseInt(inputHoras)
            rl.question('', (inputValor:string) => {
                valorPorHora = parseFloat(inputValor)

                let salario: number = horasTrabalhadas * valorPorHora
                console.log(`NUMBER = ${numeroFuncionario}`);
                console.log(`SALARY = U$ ${salario.toFixed(2)}`);
                rl.close();
            })
        })
    })
}

calcularSalario();